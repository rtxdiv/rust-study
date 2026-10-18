use std::{cell::{Cell, RefCell}, path::{Path, PathBuf}};

struct CheckJournal {
    success: Cell<u32>,
    failed: Cell<u32>,
    events: RefCell<Vec<String>>
}

impl CheckJournal {
    fn new() -> Self {
        Self { success: Cell::new(0), failed: Cell::new(0), events: RefCell::new(Vec::new()) }
    }
    fn record_results(&self, file: &Path, status: bool) {
        let status_text = match status {
            true => {
                self.success.set(self.success.get() + 1);
                "OK"
            },
            false => {
                self.failed.set(self.failed.get() + 1);
                "ERR"
            }
        };
        self.events.borrow_mut().push(format!("{} {}", status_text, file.to_string_lossy()));
    }
    fn report(&self) -> String {
        for row in self.events.borrow().iter() {
            println!("{row}");
        }
        format!("Успешно: {}\nОшибок: {}", self.success.get(), self.failed.get())
    }
}

fn main() {
    let checker = CheckJournal::new();
    checker.record_results(&PathBuf::from("autobus/busa.svg"), true);
    checker.record_results(&PathBuf::from("train/busa.jpg"), true);
    checker.record_results(&PathBuf::from("car/busa.txt"), false);
    checker.record_results(&PathBuf::from("metro/busa.wav"), true);
    checker.record_results(&PathBuf::from("trolley/busa.zip"), false);
    checker.record_results(&PathBuf::from("golfcar/busa.gpg"), true);
    println!("\n{}", checker.report());
}
