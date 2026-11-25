use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Interfaz {
    nombre: String,
    mtu: u32,
    activa: bool,
}

fn main() {
    let json = r#"
        {
            "nombre": "eth0",
            "mtu": 1500,
            "activa": true
        }
    "#;

    match serde_json::from_str::<Interfaz>(json) {
        Ok(interfaz) => println!("{:?}", interfaz),
        Err(error) => println!("Error al deserializar: {}", error),
    }
}

