fn main() {
    let puertos = [80, 443, 8080];

    let hay_puerto_seguro = puertos.iter().any(|puerto| *puerto == 443);

    println!("{}", hay_puerto_seguro);
}
