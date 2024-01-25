use env_logger::Env;

use std::{thread, time};
use chrono::{DateTime,Local};


// The available log levels, in increasing order of severity, are "trace," "debug," "info," "warn," and "error."
pub fn init_logger(log_level: &'static str) {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", log_level) // log level prints the level inputted and above
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);
}

// sleeps in ms time
pub fn sleep (ms: u64) {
    thread::sleep(time::Duration::from_millis(ms));
}

// returns HH:MM:SS time as a String
pub fn string_system_time() -> String {
    let now = chrono::offset::Local::now();
    let dt: DateTime<Local> = now.clone().into();

    dt.format("%H:%M:%S").to_string()
}

// thanks https://stackoverflow.com/questions/38461429/how-can-i-truncate-a-string-to-have-at-most-n-characters
pub fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}