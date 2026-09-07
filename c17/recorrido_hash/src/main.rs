use std::collections::HashMap;

fn main() {
    let mut tabla = HashMap::new();

    tabla.insert("HTTP", 80);
    tabla.insert("HTTPS", 443);
    tabla.insert("SSH", 22);

    // Recorremos claves y valores.
    for (protocolo, puerto) in tabla.iter() {
        println!("{protocolo}: {puerto}");
    }
    println!();

    // Recorremos únicamente las claves.
    for protocolo in tabla.keys() {
        println!("{protocolo}");
    }
    println!();

    // Recorremos únicamente los valores.
    for puerto in tabla.values() {
        println!("{puerto}");
    }
    println!();
}

