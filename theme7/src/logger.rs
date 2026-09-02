pub mod enums;
// use crate::logger::LogLevel; - от main.rs
// use self::enums::LogLevel - от себя;
use enums::LogLevel;

pub fn log(msg: &str, level: LogLevel) {
    match level {
        LogLevel::Info => println!("[INFO] {msg}"),
        LogLevel::Warning => println!("[WARNING] {msg}"),
        LogLevel::Critical => println!("[CRITICAL] {msg}")
    }
}