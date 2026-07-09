fn main() {
    let mut direcciones = vec!["192.168.1.1"];

    println!("Direcciones: {:?}", direcciones);
    match direcciones.pop() {
        Some(d) => println!("Extraemos: {}", d),
        None => println!("No quedan direcciones que extraer"),
    }
    println!("Direcciones: {:?}", direcciones);

    match direcciones.pop() {
        Some(d) => println!("Extraemos: {}", d),
        None => println!("No quedan direcciones que extraer"),
    }
    println!("Direcciones: {:?}", direcciones);
}
