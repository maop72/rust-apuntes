use serde_json::Value;
use std::collections::HashMap;

fn procesar(datos: HashMap<String, Value>) {
    for (clave, valor) in &datos {
        println!("{}: {}", clave, valor);
    }
}

fn main() {
    let json = r#"
    {
        "nombre": "eth0",
        "mtu": 1500,
        "activa": true
    }
    "#;

    match serde_json::from_str::<HashMap<String, Value>>(json) {
        Ok(datos) => procesar(datos),
        Err(error) => println!("Error al deserializar: {}", error),
    }
}
