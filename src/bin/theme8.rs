use crate::system_monitor::analyze_cpu;

mod system_monitor {
    mod metrics {
        fn clamp_percentage(val: f64) -> f64 {
            val.clamp(0.0, 100.0)
        }

        pub struct CpuStats {
            pub label: String,
            pub core_count: u8,
            usage_pct: f64,
        }

        impl CpuStats {
            pub(crate) fn new(label: &str, cores: u8, usage: f64) -> Self {
                CpuStats {
                    label: String::from(label),
                    core_count: cores,
                    usage_pct: usage
                }
            }

            // он виден везде где виден self (CpuStats)
            pub fn usage(&self) -> f64 {
                clamp_percentage(self.usage_pct)
            }
            // он виден только в super (system_monitor)
            pub(super) fn is_critical(&self) -> bool {
                self.usage_pct > 85.0
            }
        }
    }

    pub use metrics::CpuStats;
    // делает CpuStats из приватного metrics частью system_monitor

    pub fn analyze_cpu(stats: &CpuStats) {
        println!("Label: {}\nCores: {}\nUsage: {}%",
            stats.label, stats.core_count, stats.usage());
        if stats.is_critical() {
            println!("[!] Критическая нагрузка!")
        };
    }
}

fn main() {
    use crate::system_monitor::CpuStats;
    let processor = CpuStats::new("AMD Ryzen 5 7800X", 8, 142.0);
    println!("{}\n", processor.label);
    analyze_cpu(&processor);
}