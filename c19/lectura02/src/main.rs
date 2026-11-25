use std::fs;
use std::io;

fn leer_fichero(nombre: &str) -> io::Result<String> {
    let contenido = fs::read_to_string(nombre)?;
    Ok(contenido)
}

fn main() {
    let nombre = "/tmp/mensaje.txt";

    match leer_fichero(nombre) {
        Ok(contenido) => {
            println!("Contenido del fichero:");
            print!("{}", contenido);
        }
        Err(error) => {
            println!("No se pudo leer el fichero: {}", error);
        }
    }
}
