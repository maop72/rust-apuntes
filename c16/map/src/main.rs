fn main() {
    let puertos = [80, 443, 8080];
    
    let nombres = puertos
        .iter()
        .map(|puerto| format!("Puerto {}", puerto));

    for nombre in nombres {
        println!("{}", nombre);
    }
}
