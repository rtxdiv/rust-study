use std::collections::{BTreeMap};
use chrono::{NaiveDate, Datelike};

const MONTHS: [&str; 12] = ["января", "февраля", "марта", "апреля", "мая", "июня", "июля", "августа", "сентября", "октября", "ноября", "декабря"];

#[derive(Debug)]
struct Data {
    temperature: f64,
    humidity: f64,
    error: Option<String>
}

fn process_sensor_stack(stack: &mut BTreeMap<NaiveDate, Data>) {
    while let Some((date, data)) = stack.pop_first() {
        println!("{} {} {}г.", date.day(), MONTHS[(date.month()-1) as usize], date.year());
        if let Some(error) = data.error {
            println!("[!] Произошёл сбой датчика\n{error}");
            println!();
            continue
        }
        println!("Температура: {}\nВлажность: {}", data.temperature, data.humidity);
        if data.temperature > 30.0 { println!("[…] Жаркий день") }
        println!();
    }
}

fn main() {
    let mut timedata: BTreeMap<NaiveDate, Data> = BTreeMap::new();
    timedata.insert(NaiveDate::from_ymd_opt(2026, 8, 25).unwrap(),
        Data { temperature: 22.5, humidity: 45.0, error: None }
    );
    timedata.insert(NaiveDate::from_ymd_opt(2026, 8, 26).unwrap(),
        Data { temperature: 30.1, humidity: 60.5, error: None }
    );
    timedata.insert(NaiveDate::from_ymd_opt(2026, 8, 27).unwrap(),
        Data { temperature: 0.0, humidity: 55.2, error: Some(String::from("Ошибка получения температуры")) }
    );
    timedata.insert(NaiveDate::from_ymd_opt(2026, 8, 28).unwrap(),
        Data { temperature: 15.3, humidity: 92.1, error: None }
    );
    process_sensor_stack(&mut timedata);
}