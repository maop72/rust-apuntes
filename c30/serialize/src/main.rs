use serde::Serialize;
use serde_json::to_string_pretty;

#[derive(Serialize)]
struct Interfaz {
    nombre: String,
    mtu: u32,
    activa: bool,
}

fn main() {
    let interfaz = Interfaz {
        nombre: String::from("eth0"),
        mtu: 1500,
        activa: true,
    };

    match to_string_pretty(&interfaz) {
        Ok(json) => println!("{}", json),
        Err(error) => println!("Error al serializar: {}", error),
    }
}
