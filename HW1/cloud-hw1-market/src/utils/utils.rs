use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt::Write;

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
    let now = SystemTime::now();
    let duration = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    let total_seconds = duration.as_secs();
    let days_since_epoch = total_seconds / 86400;
    let seconds_in_day = total_seconds % 86400;
    let hours = (seconds_in_day / 3600 + 8)%24;
    let minutes = (seconds_in_day % 3600) / 60;
    let seconds = seconds_in_day % 60;

    let unix_days_offset = 719_468;
    let z = days_since_epoch + unix_days_offset;
    let era = z / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let year = yoe + era * 400;

    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let mp = mp as i64;
    let month = mp + if mp < 10 { 3 } else { -9 };

    let mut formatted_time = String::new();
    write!(
        &mut formatted_time,
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        year, month, day, hours, minutes, seconds
    )
    .unwrap();

    formatted_time
}
