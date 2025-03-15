use crate::models::user::{get_users};

pub fn register(username: &str) -> String {
    let users = get_users().lock().unwrap();

    if users.contains(&username.to_lowercase()) {
        return "Error - user already exists".to_string();
    }

    drop(users);
    let mut users = get_users().lock().unwrap();
    users.insert(username.to_lowercase());

    "Success".to_string()
}
