use std::fs;

fn main() {
    let texto = "Rust es un lenguaje moderno.";

    fs::write("/tmp/mensaje.txt", texto).expect("Error al escribir el fichero");

    println!("Fichero escrito correctamente.");
}
