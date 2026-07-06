enum Mensaje {
    Texto(String),
    Numero(i32),
    Salir,
}

fn main() {
    let mensaje = Mensaje::Numero(42);

    if let Mensaje::Texto(texto) = mensaje {
        println!("Texto recibido: {}", texto);
    }
}
