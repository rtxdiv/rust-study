use std::{sync::{Arc, Mutex}, thread::{self, JoinHandle, Thread}, time::Duration};

fn main() {
    println!("Main thread\n");
    let text = Arc::new(String::from("sharedtext"));
    let thread_text = Arc::clone(&text);

    let handle: JoinHandle<()> = thread::Builder::new().name("nameeee".into()).spawn(move || {
        thread::park();
        // это небезопасно, токен может появиться раньше
        thread::sleep(Duration::from_millis(500));
        println!("-- Additional thread: {thread_text}");
    }).unwrap();

    println!("Starting additional thread with text \"{text}\"");
    let additional_thread: &Thread = handle.thread();
    println!("name: {:?}", additional_thread.name());
    println!("id: {:?}", additional_thread.id());
    additional_thread.unpark();
    handle.join().unwrap();
    println!();

    let text: Arc<Mutex<String>> = Arc::new(Mutex::new(String::from("sometext")));
    thread::scope(|scope| {
        let text_ref = &text;
        scope.spawn(move || {
            thread::sleep(Duration::from_millis(1000));
            text_ref.clone().lock().unwrap().push_str("_thread1!");
            println!("thread №1 completed");
        });
        scope.spawn(move || {
            thread::sleep(Duration::from_millis(500));
            text_ref.clone().lock().unwrap().push_str("_thread2!");
            println!("thread №2 completed");
        });
        // в thread::scope автоматичексий .join()
    });
    println!("-- Result: {}", text.lock().unwrap());
}
