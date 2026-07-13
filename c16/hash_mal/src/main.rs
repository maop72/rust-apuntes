use std::collections::HashMap;

fn main() {
    let mut tabla = HashMap::new();

    tabla.insert("00:11:22:33:44:55", "192.168.1.10");
    tabla.insert("AA:BB:CC:DD:EE:FF", "192.168.1.20");

    // Lectura mediante el operador []
    println!(
        "IP: {}",
        tabla["00:11:22:33:44:55"]
    );

    // El operador [] no puede utilizarse para modificar un elemento.
    tabla["00:11:22:33:44:55"] = "192.168.1.100"; // ¡Error!
}
