enum Mensaje {
    Texto(String),
    Numero(i32),
    Salir,
}

fn main() {
    let mensaje = Mensaje::Texto(String::from("Hola"));

    match mensaje {
        Mensaje::Texto(texto) => {
            println!("Texto recibido: {}", texto);
        }
        Mensaje::Numero(n) => {
            println!("Número recibido: {}", n);
        }
        Mensaje::Salir => {
            println!("Fin del programa");
        }
    }
}
