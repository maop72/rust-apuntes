const ULTIMO_PUERTO_PRIV: u32 = 1023;
fn main() {
    let puertos = [80, 443, 8080, 3306, 22];

    let puertos_priv = puertos
        .iter()
        .filter(|puerto| **puerto > ULTIMO_PUERTO_PRIV);

    for puerto in puertos_priv {
        println!("{}", puerto);
    }
}
