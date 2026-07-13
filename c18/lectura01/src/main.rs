use std::fs;

fn main() {
    let contenido = fs::read_to_string("/tmp/mensaje.txt")
        .expect("Error al leer el fichero");

    println!("{}", contenido);
}

