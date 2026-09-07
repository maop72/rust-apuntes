fn main() {
    let puertos = [80, 443, 8080];

    for puerto in puertos.iter() {
        print!("{} ", puerto);
    }
    println!();

    // O lo que es lo mismo:
    for puerto in &puertos {
        print!("{} ", puerto);
    }
}
