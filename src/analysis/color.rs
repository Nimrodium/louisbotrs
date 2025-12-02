use crate::database::user::User;
use crate::{config::Config, database::epoch::LouisEpoch};
use rand::{self, Rng};
use std::{
    collections::{HashMap, hash_map::Entry},
    fs::File,
    path::{Path, PathBuf},
};
pub struct ColorConfig {
    // cfg: Config,
    path: PathBuf,
    data: HashMap<String, String>,
}
impl ColorConfig {
    pub fn new(database_directory: &Path) -> Result<Self, String> {
        let path = database_directory.join("colors.json");
        Self {
            data: {
                if path.exists() {
                    serde_json::from_reader(
                        File::open(&path).map_err(|e| format!("failed to open {path:?}: {e}"))?,
                    )
                    .map_err(|e| format!("could not read {path:?}: {e}"))?
                } else {
                    HashMap::new()
                }
            },
            path: path,
        };
        todo!()
    }
    pub fn flush(&self) -> Result<(), String> {
        serde_json::to_writer(
            File::open(&self.path).map_err(|e| format!("failed to open {:?}: {e}", self.path))?,
            &self.data,
        )
        .map_err(|e| format!("could not serialize {:?}: {e}", self.path))
    }
    fn get_color(&self, user_id: usize) -> Option<&String> {
        self.data.get(&user_id.to_string())
    }
    fn set_color(&mut self, user_id: usize, color: &str) {
        self.data.insert(user_id.to_string(), color.to_string());
    }
    fn get_colors<'a>(&self, users: &'a [User]) -> Vec<(&'a User, String)> {
        users
            .iter()
            .map(|u| {
                (
                    u,
                    self.get_color(u.id as usize)
                        .map(|c| c.to_string())
                        .unwrap_or(rand::rng().random_range(0x333333..0xdddddd).to_string()),
                )
            })
            .collect()
    }
}
