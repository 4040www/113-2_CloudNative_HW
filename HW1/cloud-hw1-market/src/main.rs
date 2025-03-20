mod commands;
mod services;
mod models;
mod utils;

use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    // Main loop : read commands from stdin and execute
    loop {
        print!("# ");
        stdout.flush().unwrap();

        let mut input = String::new();

        // Read commands until an empty line is encountered
        while let Ok(line) = stdin.lock().lines().next().unwrap() {
            if line.trim().is_empty() {
                break;
            }
            input.push_str(&line);
            input.push('\n');
        }

        let commands: Vec<&str> = input.trim().split('\n').collect();
        
        // Execute each command
        for command in commands {
            let trimmed = command.trim();
            if trimmed.is_empty() {
                continue;
            }

            let response = match trimmed.split_whitespace().next().unwrap_or("") {
                "REGISTER" => commands::register::execute(trimmed.to_string()),
                "CREATE_LISTING" => commands::create_listing::execute(trimmed.to_string()),
                "DELETE_LISTING" => commands::delete_listing::execute(trimmed.to_string()),
                "GET_LISTING" => commands::get_listing::execute(trimmed.to_string()),
                "GET_CATEGORY" => commands::get_category::execute(trimmed.to_string()),
                "GET_TOP_CATEGORY" => commands::get_top_category::execute(trimmed.to_string()),
                _ => "Error - unknown command".to_string(),
            };

            println!("{}", response);
        }
    }
}

