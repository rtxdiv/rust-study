enum OrderStatus {
    New,
    Processing { priority: u8 },
    Delivered { days_ago: u32 },
    Cancelled(String)
}

fn evaluate_order(status: &OrderStatus) -> String {
    match status {
        OrderStatus::New => "Новый заказ, ожидает обработки".into(),
        OrderStatus::Processing { priority: 0..=2 } => "Низкий приоритет".into(),
        OrderStatus::Processing { priority: p @ 3..=10 } => format!("Высокий приоритет: {p}"),
        OrderStatus::Processing { priority: 11_u8..=u8::MAX } => "Экстренный приоритет!".into(),
        OrderStatus::Delivered { days_ago: 0 } => "Доставлен сегодня".into(),
        OrderStatus::Delivered { days_ago: d @ 1..=30 } => format!("Доставлен {d} дн. назад"),
        OrderStatus::Delivered { days_ago: 31_u32..=u32::MAX } => "Архивный заказ".into(),
        OrderStatus::Cancelled(reason) if reason == "out of stock" => "Отменен: нет на складе".into(),
        OrderStatus::Cancelled(reason) => format!("Отменен по причине: {reason}")
    }
}

fn main() {
    println!("{}", evaluate_order(&OrderStatus::New));
    println!("{}", evaluate_order(&OrderStatus::Processing { priority: 4 }));
    println!("{}", evaluate_order(&OrderStatus::Delivered { days_ago: 2 }));
    println!("{}", evaluate_order(&OrderStatus::Cancelled(String::from("out of stock"))));
    
}