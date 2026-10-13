use chrono::Local;

fn main() {
    let ahora = Local::now();

    println!("Fecha y hora: {}", ahora.format("%Y-%m-%d %H:%M:%S"));
    println!("Formato ISO 8601: {}", ahora.to_rfc3339());
}
