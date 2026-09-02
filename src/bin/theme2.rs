use std::collections::{HashMap, HashSet, hash_map::Entry};

fn count_words(text: &str) -> HashMap<String, usize> {
    let words = text.split_whitespace();
    let mut map: HashMap<String, usize> = HashMap::new();
    for word in words {
        // не хочу .or_insert(), нужно тренироваться
        match map.entry(word.to_lowercase()) {
            Entry::Occupied(mut occ) => {
                let count = occ.get_mut();
                *count += 1;
            }
            Entry::Vacant(vac) => {
                vac.insert(1);
            }
        }
    }
    map
}

fn get_common_tags(tags1: &[&str], tags2: &[&str]) -> HashSet<String> {
    let set1: HashSet<&str> = tags1.iter().copied().collect();
    let set2: HashSet<&str> = tags2.iter().copied().collect();
    set1.intersection(&set2).map(|&e| e.to_string()).collect()
}

fn main() {
    // TASK 1
    let result = count_words("some text aaaaa text some text");
    println!("Подсчёт слов: {result:?}");

    // TASK 2
    let result = get_common_tags(&["i", "use", "arch", "btw"], &["arch", "is", "a", "sh#t"]);
    println!("Совпадающие слова: {result:?}");
}