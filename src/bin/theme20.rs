use std::{ops::{Deref, DerefMut}};
use std::any::type_name_of_val;

#[derive(Debug)]
enum DerefChain {
    Next(Box<DerefChain>),
    Text(String)
}

impl DerefChain {
    fn new(len: u16, text: String) -> Self {
        let mut chain = DerefChain::Text(text);
        for _ in 0..len {
            chain = DerefChain::Next(Box::new(chain));
        }
        chain
    }
}

impl Deref for DerefChain {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Next(deref_chain) => {
                print!("Deref -> ");
                deref_chain.deref()
            },
            Self::Text(text) => {
                println!("{}: {text}", type_name_of_val(&text));
                text
            }
        }
    }
}

impl DerefMut for DerefChain {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            DerefChain::Next(deref_chain) => {
                print!("DerefMut -> ");
                deref_chain.deref_mut()
            },
            DerefChain::Text(text) => {
                println!("{}: {text}", type_name_of_val(&text));
                text
            },
        }
    }
}

impl Drop for DerefChain {
    fn drop(&mut self) {
        let text = self.deref_mut();
        text.push('+');
        println!("Drop\n");
    }
}

struct CoolString(String);

impl Deref for CoolString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

impl DerefMut for CoolString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_str()
    }
}

fn main() {
    // TASK 1
    let mut cool_string = CoolString("rust".into());
    println!("{}", &*cool_string);
    // & (* (cool_string.deref() -> &str) -> str) -> &str
    cool_string.make_ascii_uppercase();
    // cool_string.deref_mut().make_ascii_uppercase();
    println!("{}", &*cool_string);
    println!();

    // TASK 2
    let mut chain = DerefChain::new(3, "iusearchbtw".into());
    println!("CHAIN: {chain:?}\n");

    let _ = chain.deref();
    println!();

    chain.deref_mut().make_ascii_uppercase();
    println!("{chain:?}\n");

    let owner: String = std::mem::replace(chain.deref_mut(), String::from("newtext"));
    println!("{chain:?}\n");

    println!("New owner: {owner} ({})\n", type_name_of_val(&owner));
    println!();
}
