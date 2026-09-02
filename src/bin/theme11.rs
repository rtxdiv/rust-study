// если входная ссылка одна, время жизни и так определено
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or_default()
}

struct Document<'a> {
    text: &'a str
}

impl<'a> Document<'a> {
    // если входных ссылок несколько, но одна из них &self, время жизни self присваивается всем выходным ссылкам
    fn get_header(&self, max_len: usize) -> &str {
        if self.text.len() > max_len {
            &self.text[..max_len]
        } else {
            self.text
        }
    }
}

fn main() {
    println!("{}", first_word("linux > window"));
    let doc = Document { text: "some teeext424242" };
    println!("{}", doc.get_header(10))
}