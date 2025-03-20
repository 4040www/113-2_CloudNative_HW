use crate::services::listing_service::get_category;

pub fn execute(command: String) -> String {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.len() != 3 {
        return "Error - Invalid command format".to_string();
    }

    let username = parts[1];
    let category = parts[2];
    get_category(username, category)
}
