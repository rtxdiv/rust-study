struct LoggerConfig<'a> {
    default_tag: &'a str
}

struct LogEntry<'a> {
    tag: &'a str,
    message: &'a str
}

impl<'a> LogEntry<'a> {
    fn new(config: &'a LoggerConfig<'a>, custom_tag: Option<&'a str>, message: &'a str  ) -> Self {
        LogEntry { tag: resolve_tag(config, custom_tag), message }
    }
    fn render(&self) -> String {
        format!("[{}] {}", self.tag, self.message)
    }
}

fn resolve_tag<'a>(config: &'a LoggerConfig, custom_tag: Option<&'a str>) -> &'a str {
    match custom_tag {
        Some(value) => value,
        None => config.default_tag
    }
}


struct Document<'tit: 'con, 'con> {
    title: &'tit str,
    content: &'con str
}

impl<'tit, 'con> Document<'tit, 'con> {
    fn new(title: &'tit str, content: &'con str) -> Self {
        Document { title, content }
    }

    fn preview(&self) -> &'con str {
        if self.title.is_empty() {
            &self.content[0..10]
        } else {
            self.title
        }
    }
}


fn main() {
    // TASK 1
    let logger_config = LoggerConfig { default_tag: "SYS" };

    let message = String::from("some message");
    let tag = String::from("APP");
    let log_entry1 = LogEntry::new(&logger_config, Some(&tag), &message);

    let log_entry2 = LogEntry::new(&logger_config, None, "some message");
    println!("{}", log_entry1.render());
    println!("{}", log_entry2.render());

    // TASK 2
    let doc1 = Document::new("", "some contentttt");
    println!("{}", doc1.preview());

    let title = String::from("cool title");
    let content = String::from("это реально безмие");
    let doc2 = Document::new(&title, &content);
    println!("{}", doc2.preview());
}