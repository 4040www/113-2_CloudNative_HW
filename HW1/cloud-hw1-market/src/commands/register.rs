use crate::services::user_service::register;

pub fn execute(command: String) -> String {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.len() != 2 {
        return "Error - Invalid command format".to_string();
    }

    let username = parts[1];
    register(username)
}
