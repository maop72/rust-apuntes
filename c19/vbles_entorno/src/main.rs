use std::env;

fn main() {
    match env::var("HOME") {
        Ok(valor) => println!("HOME = {valor}"),
        Err(error) => println!("Error al leer HOME: {error}"),
    }
}
