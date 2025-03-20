mod commands;
mod services;
mod models;
mod utils;

use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::io::{self, Write};

fn main() {
    let mut rl = DefaultEditor::new().unwrap();

    loop {
        let readline = rl.readline("# ");

        match readline {
            Ok(lines) => {
                let trimmed = lines.trim();
                if trimmed.is_empty() {
                    continue;
                }

                for line in trimmed.lines() {
                    let command = line.split_whitespace().next().unwrap_or("");
                    let response = match command {
                        "REGISTER" => commands::register::execute(line.to_string()),
                        "CREATE_LISTING" => commands::create_listing::execute(line.to_string()),
                        "DELETE_LISTING" => commands::delete_listing::execute(line.to_string()),
                        "GET_LISTING" => commands::get_listing::execute(line.to_string()),
                        "GET_CATEGORY" => commands::get_category::execute(line.to_string()),
                        "GET_TOP_CATEGORY" => commands::get_top_category::execute(line.to_string()),
                        _ => "Error - unknown command".to_string(),
                    };

                    println!("{}", response);
                    io::stdout().flush().unwrap();
                }

                rl.add_history_entry(lines).unwrap();
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
