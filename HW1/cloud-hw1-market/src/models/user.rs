use std::sync::{Mutex, OnceLock};
use std::collections::HashSet;

#[derive(Debug, Clone)] 
pub struct User {
    pub username: String,
}

static USERS: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

pub fn get_users() -> &'static Mutex<HashSet<String>> {
    USERS.get_or_init(|| Mutex::new(HashSet::new()))
}
 