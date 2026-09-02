mod logger;
// подключить модуль
// use self::logger::enums::LogLevel; - от себя
use logger::enums::LogLevel;

fn main() {
    logger::log("я устал", LogLevel::Info);
    logger::log("с меня достаточно", LogLevel::Warning);
    logger::log("я удаляю rust!!", LogLevel::Critical);
}