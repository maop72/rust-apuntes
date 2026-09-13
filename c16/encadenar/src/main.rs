const ULTIMO_PUERTO_PRIV: u32 = 1023;
fn main() {
    let puertos = [80, 443, 8080, 3306, 22];

    let nombres = puertos
        .iter()
        .filter(|puerto| **puerto > ULTIMO_PUERTO_PRIV)
        .map(|puerto| format!("Puerto {}", puerto));

    println!("Puertos de usuario:");
    for nombre in nombres {
        println!("{}", nombre);
    }
}
