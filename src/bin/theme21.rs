use std::{sync::Arc, thread::{self, JoinHandle, Thread}, time::Duration};

fn main() {
    println!("Main thread\n");
    let text = Arc::new(String::from("shared text"));
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

    let text = String::from("Быстрее оказалось: ");
    thread::scope(|scope| {
        let text_ref = &text;
        scope.spawn(move || {
            println!("{text_ref}");
        });
        scope.spawn(move || {
            println!("{text_ref}")
        });
    })
}
