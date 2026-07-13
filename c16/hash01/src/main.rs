use std::collections::HashMap;

fn main() {
    let mut tabla = HashMap::new();

    // Insertamos dos elementos.
    tabla.insert("00:11:22:33:44:55", "192.168.1.10");
    tabla.insert("AA:BB:CC:DD:EE:FF", "192.168.1.20");

    // Mostramos la tabla completa.
    println!("{:?}", tabla);   

    // Mostramos el elemento asociado a una clave
    println!("IP: {}", tabla.get("00:11:22:33:44:55").unwrap());

    // Reemplazamos el valor de un elemento
    tabla.insert("00:11:22:33:44:55", "192.168.1.100");

    // Eliminamos un elemento
    tabla.remove("AA:BB:CC:DD:EE:FF");

    println!("{:?}", tabla);   
}
