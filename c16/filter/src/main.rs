fn main() {
    let puertos = [80, 443, 8080, 3306, 22];

    let puertos_altos = puertos
        .iter()
        .filter(|puerto| **puerto > 1024);

    for puerto in puertos_altos {
        println!("{}", puerto);
    }
}

