use std::collections::{BTreeMap, VecDeque};

fn process_task_queue(queue: &mut VecDeque<&str>) {
    queue.push_back("task_3");
    queue.push_front("urgent_task");
    if let Some(value) = queue.pop_front() {
        println!("Первый элемент: {value}")
    }
    let slice: &[&str] = queue.make_contiguous();
    println!("Соединенный срез: {slice:?}");
}

fn filter_events_by_timestamp(events: &BTreeMap<u64, String>, min: u64, max: u64) {
    for (timestamp, event) in events.range(min..=max) {
        println!("Время: {event}, Событие: {timestamp}");
    }
}

fn main() {
    // TASK 1
    let mut queue: VecDeque<&str> = VecDeque::from(vec!["task_1", "task_2"]);
    process_task_queue(&mut queue);
    println!();
    
    // TASK 2
    let mut map: BTreeMap<u64, String> = BTreeMap::new();
    map.insert(100, String::from("Boot"));
    map.insert(250, String::from("User login"));
    map.insert(400, String::from("Error 404"));
    map.insert(500, String::from("Shutdown"));
    filter_events_by_timestamp(&map, 200, 450);
}