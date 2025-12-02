use std::{
    collections::{HashMap, hash_map::Entry},
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use chrono::{Datelike, Timelike};
use serde_derive::{Deserialize, Serialize};

use crate::database::{
    epoch::{LouisEpoch, UnixEpoch, unix_to_epoch},
    user::User,
};
pub type UserUpdate<'a> = (usize, &'a str, usize, &'a [(&'a str, usize)], UnixEpoch);

use super::epoch::now_louis_epoch;

#[derive(Serialize, Deserialize)]
struct ServerFileInit {
    users: HashMap<u64, User>,
    reactions: Vec<String>,
    meta: HashMap<String, u64>,
}
impl ServerFileInit {
    fn from_server_file(from: ServerFile) -> Self {
        Self {
            users: from.users,
            reactions: from.reactions,
            meta: from.meta,
        }
    }
    fn to_server_file(self, path: PathBuf, read_only: bool) -> ServerFile {
        ServerFile {
            path,
            users: self.users,
            reactions: self.reactions,
            meta: self.meta,
            read_only,
        }
    }
}
#[derive(Clone)]
pub struct ServerFile {
    path: PathBuf,
    users: HashMap<u64, User>,
    reactions: Vec<String>,
    meta: HashMap<String, u64>,
    read_only: bool,
}
impl ServerFile {
    pub fn new(path: &str) -> Self {
        Self {
            path: PathBuf::from(path),
            users: HashMap::new(),
            reactions: Vec::new(),
            meta: HashMap::new(),
            read_only: false,
        }
    }
    /// load the server file from specified path
    pub fn load(path: &Path, read_only: bool) -> Result<Self, String> {
        // let path = PathBuf::from(path);
        // let raw_json = String::new();
        let mut file =
            File::open(&path).map_err(|e| format!("failed to load Server \"{path:?}\": {e}"))?;
        // file.read_to_string(&mut raw_json);
        serde_json::from_reader(file)
            .map(|a: ServerFileInit| a.to_server_file(path.to_path_buf(), read_only))
            .map_err(|e| format!("could not parse server file \"{path:?}\": {e}"))
    }
    // maybe flush should consume self, then you reinit
    // or just force ServerFileInit to have explicit lifetimes
    fn flush(&self) -> Result<(), String> {
        if self.read_only {
            Err("attempted to flush a read only file".to_string())
        } else {
            let serialized = serde_json::to_string(&ServerFileInit::from_server_file(self.clone()))// clone here is bad.
                .map_err(|e| format!("could not serialize server {:?}: {e}", self.path))?;
            fs::create_dir_all(&self.path)
                .map_err(|e| format!("could not create directory {:?}: {e}", self.path))?;
            let mut file = File::open(&self.path)
                .map_err(|e| format!("could not open file {:?}: {e}", self.path))?;

            file.write(serialized.as_bytes())
                .map_err(|e| format!("could not write to file {:?}: {e}", self.path))?;
            Ok(())
        }
    }
    fn get_user(&self, id: usize) -> Option<&User> {
        self.users.get(&(id as u64))
    }
    fn get_mut_user(&mut self, id: usize) -> Option<&mut User> {
        self.users.get_mut(&(id as u64))
    }
    fn create_user(&mut self, id: usize, name: &str) {
        self.users.insert(id as u64, User::new(id as u64, name));
    }
    fn get_or_create_user(&mut self, id: usize, name: &str) -> &mut User {
        match self.users.entry(id as u64) {
            Entry::Occupied(occupied_entry) => occupied_entry.into_mut(),
            Entry::Vacant(vacant_entry) => vacant_entry.insert(User::new(id as u64, name)),
        }
    }
    fn get_all_users(&self) -> Vec<&User> {
        self.users.values().collect()
    }
    fn get_all_reactions(&self) -> &[String] {
        &self.reactions
    }
    fn update_last_day(&mut self, day: LouisEpoch) {
        // self.meta.get_mut("last_day").map(|d|)
        self.meta.insert("last_day".to_string(), day);
    }
    fn update_last_day_now(&mut self) {
        self.update_last_day(now_louis_epoch())
    }
    fn file_name(server_name: &str, year: &str) -> PathBuf {
        PathBuf::from(format!("{server_name}_{year}.json"))
    }
    fn file_path(server_name: &str, year: &str) -> PathBuf {
        let mut a = PathBuf::from(server_name);
        a.push(Self::file_name(server_name, year));
        a
    }

    fn update_message_count(&mut self, user_id: usize, name: &str, date: &UnixEpoch, count: usize) {
        let user = self.get_or_create_user(user_id, name);
        user.update_message_count(unix_to_epoch(date), date.hour() as usize, count);
    }
    fn update_reaction_count(
        &mut self,
        user_id: usize,
        name: &str,
        date: &UnixEpoch,
        reaction: &str,
        count: usize,
    ) {
        let user = self.get_or_create_user(user_id, name);
        user.update_reaction_count(unix_to_epoch(&date), date.hour() as usize, reaction, count);
    }
}
struct ServerFiles {
    directory: PathBuf,
    server_name: String,
    files: HashMap<usize, ServerFile>,
}
impl<'a> ServerFiles {
    fn new(directory: &Path, server_name: &str) -> Self {
        Self {
            directory: directory.to_path_buf(),
            server_name: server_name.to_string(),
            files: HashMap::new(),
        }
    }
    fn open_server(&'a mut self, year: usize) -> Result<&'a mut ServerFile, String> {
        if self.files.contains_key(&year) {
            Ok(self.files.get_mut(&year).unwrap())
        } else {
            self.files.insert(
                year,
                ServerFile::load(
                    // ServerFile::file_name(&self.server_name.as_str(), &year.to_string()),
                    &ServerFile::file_path(&self.server_name, &year.to_string()),
                    false,
                )?,
            );
            Ok(self.files.get_mut(&year).unwrap())
        }
    }
}

pub struct ServerDatabase {
    path: PathBuf,
    database: ServerFiles,
}
impl ServerDatabase {
    pub fn new(path: &Path) -> Result<Self, String> {
        Ok(Self {
            path: path.to_path_buf(),
            database: ServerFiles::new(
                path.parent().ok_or(format!(
                    "Invalid server path {path:?} does not contain a parent"
                ))?,
                path.file_name()
                    .ok_or(format!(
                        "Invalid server path {path:?} does not contain a basename"
                    ))?
                    .to_str()
                    .ok_or(format!("could not convert OsStr to Str in {path:?}"))?,
            ),
        })
    }
    pub fn update_users(
        &mut self,
        // date: UnixEpoch,
        data: &[UserUpdate],
    ) -> Result<(), String> {
        // let server = self.database.open_server(date.year() as usize)?;
        for (id, name, messages, reactions, date) in data {
            let server = self.database.open_server(date.year() as usize)?;
            server.update_message_count(*id, name, date, *messages);
            for (reaction, count) in *reactions {
                server.update_reaction_count(*id, name, date, reaction, *count);
            }
        }
        Ok(())
    }
}
pub struct BatchCache {
    fill: usize,
    path: PathBuf,
    servers: HashMap<String, HashMap<String, f64>>,
}
impl BatchCache {
    fn new(path: &Path) -> Result<Self, String> {
        if path.exists() {
            let file = File::open(path).map_err(|e| format!("failed to open {path:?}: {e}"))?;
            let servers = serde_json::from_reader(file)
                .map_err(|e| format!("could not read {path:?}: {e}"))?;
            Ok(Self {
                fill: 0,
                path: path.to_path_buf(),
                servers,
            })
        } else {
            Ok(Self {
                fill: 0,
                path: path.to_path_buf(),
                servers: HashMap::new(),
            })
        }
    }
    fn flush(&self) -> Result<(), String> {
        serde_json::to_writer(
            File::open(&self.path).map_err(|e| format!("failed to open {:?}: {e}", self.path))?,
            &self.servers,
        )
        .map_err(|e| format!("could not serialize {:?}: {e}", self.path))
    }
    fn log_pointer(&mut self, server_id: usize, channel_id: usize, ptr: f64) {
        let server = match self.servers.entry(server_id.to_string()) {
            Entry::Occupied(occupied_entry) => occupied_entry.into_mut(),
            Entry::Vacant(vacant_entry) => vacant_entry.insert(HashMap::new()),
        };
        if let Some(channel) = server.get_mut(&channel_id.to_string()) {
            *channel = ptr;
            println!("logging pointer\n\tserver: {server_id}\n\tchannel: {channel_id}");
        }
    }
    fn clear(&mut self) {
        self.servers.clear();
    }
}
