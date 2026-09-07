// https://iterators.aloso.foo/ - конструкторы, адаптеры, потребители

#[derive(Debug, Clone)]
#[allow(unused)]
struct ServerLog {
    hostname: String,
    cpu_usage: f64,
    is_online: bool
}

fn generate_critical_alerts(logs: &[ServerLog]) -> Vec<String> {
    logs.iter()
        .filter(|log| log.is_online && log.cpu_usage > 70.0)
        .enumerate()
        .map(|(idx, log)| format!("ПРЕДУПРЕЖДЕНИЕ #{}\nХост {} загружен на {}%", idx+1, log.hostname, log.cpu_usage))
        .collect()
}


#[derive(Debug, Clone)]
#[allow(unused)]
enum Filler {
    Cycle,
    Last,
    Value(f64)
}

#[derive(Debug, Clone)]
struct Limitline {
    values: Vec<Option<f64>>,
    filler: Option<Filler>
}

impl Limitline {
    fn new(values: Vec<Option<f64>>) -> Self {
        Limitline { values, filler: None }
    }
    fn filler(&mut self, filler: Filler) {
        self.filler = Some(filler);

    }
    fn fill(&mut self, count: usize) {
        let needed = count.saturating_sub(self.values.len());
        if needed == 0 { return }

        match &self.filler {
            None => (),
            Some(Filler::Cycle) => {
                self.values.extend(self.values.clone().into_iter().cycle().take(needed));
            },
            Some(Filler::Last) => {
                if let Some(last) = self.values.last() { self.values.resize(count, *last) }
            },
            Some(Filler::Value(val)) => { self.values.resize(count, Some(*val)) }
        }
    }
}

fn analyze_load(logs: &[ServerLog], limitline: &Limitline) {
    let mut limitline = limitline.clone();
    limitline.fill(logs.len());
    let excess_logs: Vec<(usize, &ServerLog, f64)> = logs.iter()
        .zip(limitline.values.iter())
        .enumerate()
        .filter_map(|(idx, (log, limit))| {
            let value = (*limit)?;
            (log.is_online && log.cpu_usage > value).then_some((idx, log, value))
        })
        .collect();
    draw_excess_logs(&excess_logs);
    println!();

    let excess_total = excess_logs.iter().fold(0_f64, |acc, log| acc + (log.1.cpu_usage - log.2));
    println!("Суммарное превышение лимита: {excess_total}%")
}

fn draw_excess_logs(data: &Vec<(usize, &ServerLog, f64)>) {
    let widht: u32 = 30;
    let step: f64 = 100.0 / widht as f64;
    
    for (_idx, log, limit) in data {
        println!("{}", log.hostname);
        
        for i in 0..widht {
            let down = i as f64 * step;
            let top = (i + 1) as f64 * step;
            let current = log.cpu_usage;
            let limit = *limit;

            if current > down && current <= top {
                print!("\x1b[31m██\x1b[0m");
            } else if limit > down && limit <= top {
                print!("\x1b[32m██\x1b[0m");
            } else {
                print!("██");
            }
        }
        println!(" % {}/{}", log.cpu_usage, limit);
    }
}


fn main() {
    // TASK 1
    let logs = [
        ServerLog { hostname: String::from("serv1"), cpu_usage: 50.0, is_online: true },
        ServerLog { hostname: String::from("serv2"), cpu_usage: 100.0, is_online: true },
        ServerLog { hostname: String::from("serv3"), cpu_usage: 90.0, is_online: false },
        ServerLog { hostname: String::from("serv4"), cpu_usage: 70.0, is_online: true },
        ServerLog { hostname: String::from("serv5"), cpu_usage: 95.0, is_online: true },
        ServerLog { hostname: String::from("serv6"), cpu_usage: 75.0, is_online: true },
        ServerLog { hostname: String::from("serv7"), cpu_usage: 95.0, is_online: false },
        ServerLog { hostname: String::from("serv8"), cpu_usage: 60.0, is_online: true },
    ];
    let alerts = generate_critical_alerts(&logs);
    alerts.iter().for_each(|s| println!("{s}\n"));
    println!();

    // TASK 2
    let mut limitline = Limitline::new(vec![Some(70.0)]);
    limitline.filler(Filler::Last);
    analyze_load(&logs, &limitline);
}