use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

use serde_derive::{Deserialize, Serialize};

use crate::database::user::User;

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
    pub fn load(path: &str, read_only: bool) -> Result<Self, String> {
        let os_path = PathBuf::from(path);
        // let raw_json = String::new();
        let mut file =
            File::open(&os_path).map_err(|e| format!("failed to load Server \"{path}\": {e}"))?;
        // file.read_to_string(&mut raw_json);
        serde_json::from_reader(file)
            .map(|a: ServerFileInit| a.to_server_file(os_path, read_only))
            .map_err(|e| format!("could not parse server file \"{path}\": {e}"))
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
    fn create_user(&mut self, id: usize, name: &str) {
        self.users.insert(id as u64, User::new(id as u64, name));
    }
    fn get_all_users(&self) -> &[&User] {
        self.users.values().collect()
    }
    fn get_all_reactions(&self) -> &[&str]{
        self.reactions
    }
    fn update_last_day(&mut self,day:LouisEpoch){
        // self.meta.get_mut("last_day").map(|d|)
        self.meta.insert("last_day",day);
    }
    fn update_last_day_now(&mut self){
        self.update_last_day(now_louis_epoch())
    }
    fn file_name(server_name:&str,year:&str) -> PathBuf{
        format!("{server_name}_{year}.json")
    }
    fn file_path(server_name:&str,year:&str) -> PathBuf{
        PathBuf::from(server_name).push(Self::file_name(server_name, year))
    }
}
struct ServerFiles{
    directory:PathBuf,
    server_name:String,
    files:HashMap<usize,ServerFile>
} impl ServerFiles {

}
