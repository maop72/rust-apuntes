use std::io;

fn main() {
    let mut nombre = String::new();

    println!("¿Cómo te llamas?");

    io::stdin()
        .read_line(&mut nombre)
        .expect("Error al leer la entrada");

    println!("Hola, {}.", nombre.trim());
}
