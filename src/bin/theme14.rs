//  into_iter - передача владения
//  iter - передача ссылки
//  iter_mut - передача мутабельной ссылки

#[derive(Debug)]
#[allow(unused)]
struct SensorMetric {
    step: u32,
    reading: f64
}

struct MetricGenerator {
    current_step: u32,
    max_steps: u32,
    initial_value: f64
}

impl Iterator for MetricGenerator {
    type Item = SensorMetric;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_step >= self.max_steps { return None; }
        self.current_step += 1;
        let reading = self.initial_value + (self.current_step as f64 * 2.5);
        println!("Вычисление шага №{}", self.current_step);
        Some(SensorMetric { step: self.current_step, reading })
    }
}

#[allow(clippy::while_let_on_iterator)]
fn main() {
    // TASK 1
    let mut mg = MetricGenerator { current_step: 3, max_steps: 6, initial_value: 10_f64 };
    // Итератор создан, но сам ничего не делает  -  ленивые вычисления
    let metric = mg.next().unwrap_or(SensorMetric { step: 0, reading: 0_f64 });
    println!("{metric:?}");
    println!();

    // TASK 2
    // for metric in mg  -  автоматически делает mg.into_iter()
    while let Some(metric) = mg.next() {
        println!("{metric:?}")
    }
}