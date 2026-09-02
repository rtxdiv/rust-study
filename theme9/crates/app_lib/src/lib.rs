use logger::{Logger, LogLevel};

pub fn run() {
    let logger = Logger::new(LogLevel::Warning);
    
    logger.time_write("это информация", LogLevel::Info);
    logger.time_write("это предупреждение", LogLevel::Warning);
    logger.time_write("это ошибка", LogLevel::Error);
}