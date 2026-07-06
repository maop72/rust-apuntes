enum Mensaje {
    Texto(String),
    Numero(i32),
    Salir,
}

fn main() {
    let mensaje = Mensaje::Numero(42);

    match mensaje {
        Mensaje::Texto(texto) => {
            println!("Texto recibido: {}", texto);
        }
        _ => {}
    }
}
