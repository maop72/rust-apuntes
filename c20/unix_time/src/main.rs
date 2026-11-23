use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let ahora = SystemTime::now();

    let segundos = ahora
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    println!("Hora Unix: {}", segundos);
}
