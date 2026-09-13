fn main() {
    let puertos = [80, 443, 8080];
    let puerto_seguro = 443;

    let hay_puerto_seguro = puertos
        .iter()
        .any(|puerto| *puerto == puerto_seguro);

    println!("{}", hay_puerto_seguro);
}
