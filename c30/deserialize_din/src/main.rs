use serde_json::Value;

fn main() {
    let json = r#"{
        "nombre": "eth0",
        "mtu": 1500,
        "activa": true
    }"#;

    let datos: Value = serde_json::from_str(json).unwrap();

    // Si sabemos que el objeto tiene la clave 'nombre',
    // podemos conocer su valor así:
    println!("Nombre: {}\n", datos["nombre"]);

    // Si quieremos procesar todas las claves, sean 
    // las que sean, usamos as_object()
    if let Some(objeto) = datos.as_object() {
        for (clave, valor) in objeto {
            println!("{}: {}", clave, valor);
        }
    }
}
