// additions/theme13.png
use std::{sync::LazyLock};

static GLOBAL_APP_TITLE: LazyLock<&'static str> = LazyLock::new(|| {
    "очень крутое бриложение"
});

fn get_environment(env_code: u8) -> &'static str {
    match env_code {
        1 => "Production",
        2 => "Staging",
        _ => "Development"
    }
}

fn build_static_token(user_id: u64) -> &'static str {
    let row = format!("TOKEN-{}", user_id);
    let boxed = Box::new(row);
    Box::leak(boxed)
}

fn main() {
    // TASK 1
    LazyLock::force(&GLOBAL_APP_TITLE);
    println!("{}", *GLOBAL_APP_TITLE);

    // TASK 2
    let env: &'static str = get_environment(1);
    println!("{env}");

    // TASK 3
    let row: &'static str = build_static_token(42);
    println!("{row}")
}