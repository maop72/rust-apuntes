fn main() {
    let puertos = [80, 443, 8080];
    let puerto_buscado = 443;

    let resultado = puertos
        .iter()
        .position(|puerto| *puerto == puerto_buscado);

    match resultado {
        Some(indice) => println!("Encontrado en el índice {}", indice),
        None => println!("No encontrado"),
    }
}
