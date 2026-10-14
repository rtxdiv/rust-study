use std::{cell::RefCell, collections::BTreeSet, fmt::Debug, rc::{Rc, Weak}};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
struct Category {
    name: String,
    categories: BTreeSet<Rc<RefCell<Category>>>,
    themes: BTreeSet<Rc<Theme>>
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Theme {
    name: String
}

impl Category {
    fn new(name: String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { name, categories: BTreeSet::new(), themes: BTreeSet::new() }))
    }
    fn add_theme(&mut self, theme: &Rc<Theme>) {
        self.themes.insert(Rc::clone(theme));
    }
    fn add_category(&mut self, category: &Rc<RefCell<Category>>) {
        self.categories.insert(Rc::clone(category));
    }
}

impl Theme {
    fn new(name: String) -> Rc<Self> {
        Rc::new(Self { name })
    }
    fn add_to_category(self: &Rc<Self>, category: &Rc<RefCell<Category>>) {
        category.borrow_mut().add_theme(self);
    }
}


fn demonstrate_rc_sharing() {
    let row: Rc<String> = Rc::new(String::from("общий текст"));
    let strong1 = Rc::clone(&row);
    let strong2 = Rc::clone(&strong1);
    println!("Сильных ссылок: {}", Rc::strong_count(&row));
    
    let weak = Rc::downgrade(&row);
    println!("Слабых ссылок: {}", Rc::weak_count(&row));
    println!();

    weak_checker(&weak);

    drop(row);
    drop(strong1);
    drop(strong2);
    println!();
    
    weak_checker(&weak);
}

fn weak_checker(weak: &Weak<impl Debug>) {
    match weak.upgrade() {
        Some(val) => println!("Данные живы: {:?}\nСильных ссылок: {}", *val, Rc::strong_count(&val) - 1),
        None => println!("Данные 200")
    }
}


fn main() {
    // TASK 1
    demonstrate_rc_sharing();
    println!();

    // TASK 2
    let catalog = Category::new("catalog".into());

    let computers = Category::new("Компьютеры".into());
    let software = Category::new("Программное обеспечение".into());
    let linux = Category::new("Linux".into());
    let windows = Category::new("sWindows".into()); 

    catalog.borrow_mut().add_category(&computers);
    catalog.borrow_mut().add_category(&software);
    computers.borrow_mut().add_category(&software);

    software.borrow_mut().add_category(&linux);
    software.borrow_mut().add_category(&windows);

    let hardware_theme = Theme::new("Комплектующие".into());
    let software_usage_theme = Theme::new("Использование ПО".into());
    
    hardware_theme.add_to_category(&computers);
    linux.borrow_mut().add_theme(&software_usage_theme);
    windows.borrow_mut().add_theme(&software_usage_theme);

    Theme::new("Установка (Linux)".into()).add_to_category(&linux);
    Theme::new("Установка (sWindows)".into()).add_to_category(&windows);

    println!("{:#?}", catalog);
}

/*
catalog
├── Компьютеры
│   ├── Программное обеспечение
│   │   ├── Linux
│   │   │   └── [Тема] Использование ПО
│   │   │   └── [Тема] Установка (Linux)
│   │   └── sWindows
│   │       └── [Тема] Использование ПО
│   │       └── [Тема] Установка (sWindows)
│   └── [Тема] Комплектующие
└── Программное обеспечение
    ├── Linux
    │   └── [Тема] Использование ПО
    │   └── [Тема] Установка (Linux)
    └── sWindows
        └── [Тема] Использование ПО
        └── [Тема] Установка (sWindows)
*/