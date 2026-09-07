use std::collections::HashMap;

fn main() {
    let mut tabla = HashMap::new();

    tabla.insert("HTTP", 80);
    tabla.insert("HTTPS", 443);
    tabla.insert("SSH", 22);

    println!("{:?}", tabla);
    println!("¿La tabla está vacía? {}", tabla.is_empty());
    println!("Número de elementos: {}", tabla.len());
    println!("¿Contiene HTTP? {}", tabla.contains_key("HTTP"));
    println!("¿Contiene FTP? {}", tabla.contains_key("FTP"));

    println!();
    tabla.clear();
    println!("{:?}", tabla);
    println!("¿La tabla está vacía? {}", tabla.is_empty());
}
