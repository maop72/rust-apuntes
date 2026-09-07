fn main() {
    let puertos = [80, 443, 8080];

    let resultado = puertos.iter().find(|puerto| **puerto == 443);

    match resultado {
        Some(puerto) => println!("Encontrado: {}", puerto),
        None => println!("No encontrado"),
    }
}
