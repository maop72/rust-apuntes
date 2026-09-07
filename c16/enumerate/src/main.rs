fn main() {
    let puertos = [80, 443, 8080];

    for (indice, puerto) in puertos.iter().enumerate() {
        println!("Índice {}: puerto {}", indice, puerto);
    }
}

