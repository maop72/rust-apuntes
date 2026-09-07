fn main() {
    let puertos = [80, 443, 8080, 3306, 22];

    let nombres = puertos
        .iter()
        .filter(|puerto| **puerto > 1024)
        .map(|puerto| format!("Puerto {}", puerto));

    for nombre in nombres {
        println!("{}", nombre);
    }
}
