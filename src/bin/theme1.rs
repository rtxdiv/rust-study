fn process_numbers(numbers: &[i32]) -> Vec<i32> {
    let mut newvec = Vec::<i32>::new();
    for elem in numbers {
        if elem % 2 != 0 { continue }
        else { newvec.push(*elem * 2) }
    }
    newvec
}

fn clean_user_ids(ids: &mut Vec<i32>) {
    ids.retain(|x| *x >= 0 );
    ids.sort_unstable();
    ids.dedup();
}

fn print_top_three(slice: &[i32]) {
    let newslice = &slice[..3];
    println!("Первые три: {newslice:?}");
}

fn main() {
    // TASK 1
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let newvec = process_numbers(&numbers);
    println!("Преобразованный вектор: {newvec:?}");
    match newvec.first() {
        Some(value) => { println!("Первый элемент: {value}") }
        None => { println!("Нет такого") }
    }

    // TASK 2
    let mut user_ids = vec![105, -1, 42, 105, -12, 3, 42, 7, 3];
    clean_user_ids(&mut user_ids);
    print_top_three(&user_ids);
}