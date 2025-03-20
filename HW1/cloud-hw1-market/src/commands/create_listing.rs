use crate::services::listing_service::create_listing; 
use crate::utils::utils::parse_command;

pub fn execute(command: String) -> String {
    let parts = parse_command(&command);

    if parts.len() != 6 {
        return "Error - Invalid command format".to_string();
    }

    let username = &parts[1];
    let title = &parts[2];
    let description = &parts[3];
    let price: f64 = match parts[4].parse() {
        Ok(p) => p,
        Err(_) => return "Error - invalid price format".to_string(),
    };
    let category = &parts[5];

    create_listing(username, title, description, price, category)
}

