use std::{thread::{self}, time::Duration};

fn main() {
    thread::spawn(|| {
        thread::sleep(Duration::from_secs(1));
        println!("Additional thread");
    });
    println!("Main thread");
}
