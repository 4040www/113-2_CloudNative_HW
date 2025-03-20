use chrono::{Utc};

pub fn parse_command(command: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current_part = String::new();
    let mut inside_quotes = false;
    let mut chars = command.trim().chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\'' => {
                inside_quotes = !inside_quotes;
                if !inside_quotes && !current_part.is_empty() {
                    parts.push(current_part.trim().to_string());
                    current_part.clear();
                }
            },
            ' ' if !inside_quotes => {
                if !current_part.is_empty() {
                    parts.push(current_part.trim().to_string());
                    current_part.clear();
                }
            },
            _ => current_part.push(c),
        }
    }

    if !current_part.is_empty() {
        parts.push(current_part.trim().to_string());
    }

    parts
}

pub fn get_formatted_time() -> String {
    let now = Utc::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}
