mod commands;
mod services;
mod models;

use std::io::{self, BufRead};
use std::io::Write;

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        // 顯示提示符
        print!("# ");
        stdout.flush().unwrap(); // 刷新輸出緩衝區，顯示提示符

        // 讀取用戶輸入
        let line = stdin.lock().lines().next().unwrap().unwrap();
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // 擷取命令並執行
        let command = trimmed.split_whitespace().next().unwrap_or("");
        let response = match command {
            "REGISTER" => commands::register::execute(trimmed.to_string()),
            "CREATE_LISTING" => commands::create_listing::execute(trimmed.to_string()),
            _ => "Error - unknown command".to_string(),
        };

        // 顯示結果
        println!("{}", response);
    }
}
