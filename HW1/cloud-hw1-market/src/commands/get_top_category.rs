use crate::services::listing_service::get_top_category;

pub fn execute(command: String) -> String {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.len() != 2 {
        return "Error - Invalid command format".to_string();
    }

    let username = parts[1];
    get_top_category(username)
}
