use std::io::Write;

#[allow(unused)]
struct Header<'a>(&'a str);

#[allow(unused)]
struct Article<'a> {
    title: Header<'a>,
    body: &'a str
}

#[allow(unused)]
#[derive(Debug)]
enum PublicationStatus<'a> {
    Published(&'a str),
    Draft
}

trait Renderable<'a> {
    fn render_preview(&self) -> &'a str;
}

impl<'a> Renderable<'a> for Article<'a> {
    fn render_preview(&self) -> &'a str {
        if self.body.len() <= 10 {
            self.body
        } else {
            &self.body[..10]
        }
    }
}

fn main() {
    let mut row = String::new();
    print!("Введи body: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut row).unwrap();
    
    let article = Article {
        title: Header("title"),
        body: row.trim()
    };
    let status = PublicationStatus::Published("github");
    
    println!("{}", article.render_preview());
    println!("{:?}", status);
}