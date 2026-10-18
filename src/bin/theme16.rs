use std::time::Instant;

#[derive(Debug)]
struct TelemetryPacket {
    signal_dbm: i32,
    is_valid: bool
}

#[allow(clippy::needless_range_loop)]
fn process_indexed(packets: &[TelemetryPacket]) -> i64 {
    let mut sum = 0_i64;
    for i in 0..packets.len() {
        let pack = &packets[i];
        if pack.is_valid && pack.signal_dbm > -80 {
            sum += pack.signal_dbm as i64
        }
    }
    sum
}

fn process_interactive(packets: &[TelemetryPacket]) -> i64 {
    packets.iter().fold(0_i64, |acc, pack| {
        if pack.is_valid && pack.signal_dbm > -80 { acc + pack.signal_dbm as i64 }
        else { acc }
    })
}

fn main() {
    let tp = [
        TelemetryPacket { signal_dbm: 56, is_valid: true },
        TelemetryPacket { signal_dbm: 190, is_valid: true },
        TelemetryPacket { signal_dbm: 1444, is_valid: true },
        TelemetryPacket { signal_dbm: -14, is_valid: true },
        TelemetryPacket { signal_dbm: 222, is_valid: true },
        TelemetryPacket { signal_dbm: 6767, is_valid: true },
        TelemetryPacket { signal_dbm: 676767, is_valid: true },
        TelemetryPacket { signal_dbm: 34, is_valid: true },
        TelemetryPacket { signal_dbm: 42, is_valid: true },
    ];

    // TASK 1
    let start = Instant::now();
    let value = process_indexed(&tp);
    let duration = start.elapsed();
    println!("[{:?}]\n= {}", duration, value);

    // TASK 2
    let start = Instant::now();
    let value = process_interactive(&tp);
    let duration = start.elapsed();
    println!("[{:?}]\n= {}", duration, value);
}
