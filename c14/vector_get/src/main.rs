fn main() {
    let protocolos = vec!["tcp", "ip"];

    let indice = 2;

    match protocolos.get(indice) {
        Some(nombre) => println!("Protocolo: {}", nombre),
        None => println!("Índice fuera de rango"),
    }
}
