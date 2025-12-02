use crate::config::Config;
use crate::database::user::User;
use std::{
    collections::{HashMap, hash_map::Entry},
    path::PathBuf,
};
pub struct ColorConfig {
    cfg: Config,
    path: PathBuf,
    data: HashMap<String, String>,
}
impl ColorConfig {
    pub fn new() {
        todo!()
    }
    pub fn flush(&self) -> Result<(), String> {
        todo!()
    }
    fn get_color(&self, user_id: usize) -> Option<&str> {
        todo!()
    }
    fn set_color(&mut self, user_id: usize, color: &str) {
        self.data.insert(user_id.to_string(), color.to_string());
    }
    fn get_colors<'a>(&'a self, users: &'a [User]) -> Vec<(&'a User, &'a str)> {
        todo!()
    }
}
