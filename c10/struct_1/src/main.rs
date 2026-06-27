struct Ruta {
    destino: String,
    prefijo: u8,
    siguiente_salto: String,
    interface: String,
}

fn main() {
    let ruta1 = Ruta {
        destino: String::from("192.168.1.0"),
        prefijo: 24,
        siguiente_salto: String::from("10.0.0.1"),
        interface: String::from("eth0"),
    };

    println!("Destino: {}", ruta1.destino);
    println!("prefijo: {}", ruta1.prefijo);
    println!("siguiente_salto: {}", ruta1.siguiente_salto);
    println!("interface: {}", ruta1.interface);
}
