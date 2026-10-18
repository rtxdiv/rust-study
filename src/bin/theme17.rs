#[derive(Debug)]
enum Expression {
    Number(f64),
    Add(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>)
}

fn eval(expr: &Expression) -> f64 {
    match expr {
        Expression::Number(n) => *n,
        Expression::Add(n1, n2) => eval(n1) + eval(n2),
        Expression::Multiply(n1, n2) => eval(n1) * eval(n2)
    }
}

fn count_nodes(expr: &Expression) -> usize {
    match expr {
        Expression::Number(_) => 1,
        Expression::Add(left, right) | Expression::Multiply(left, right) => {
            1 + count_nodes(left) + count_nodes(right)
        }
    }
}

fn main() {
    let expr = Expression::Multiply(
        Box::new(Expression::Add(
            Box::new(Expression::Number(2.0)),
            Box::new(Expression::Number(3.0))
        )),
        Box::new(Expression::Number(4.0))
    );
    println!("Структура дерева: {:#?}", expr);

    let result = eval(&expr);
    println!("Результат: {result}");

    let count = count_nodes(&expr);
    println!("Количество узлов: {count}");
}
