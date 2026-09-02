#[cfg(feature = "time")]
use chrono::{Utc};

#[derive(PartialEq, PartialOrd)]
pub enum LogLevel {
    Info,
    Warning,
    Error
}

pub struct Logger {
    min_level: LogLevel
}

impl Logger {
    pub fn new(min_level: LogLevel) -> Self {
        Logger { min_level }
    }

    pub fn write(&self, text: &str, level: LogLevel) {
        if level >= self.min_level {
            match level {
                LogLevel::Info => println!("[INFO]: {text}"),
                LogLevel::Warning => println!("[WARN]: {text}"),
                LogLevel::Error => println!("[ERROR]: {text}"),
            }
        }
    }
    
    #[cfg(feature = "time")]
    pub fn time_write(&self, text: &str, level: LogLevel) {
        if level >= self.min_level {
            let now_str = Self::get_formatted_now();
            match level {
                LogLevel::Info => println!("{now_str} [INFO]: {text}"),
                LogLevel::Warning => println!("{now_str} [WARN]: {text}"),
                LogLevel::Error => println!("{now_str} [ERROR]: {text}"),
            }
        }
    }
    
    #[cfg(feature = "time")]
    fn get_formatted_now() -> String {
        Utc::now().format("%d.%m.%Y, %H:%M:%S").to_string()
    }
}