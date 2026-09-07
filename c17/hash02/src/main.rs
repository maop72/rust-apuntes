use std::collections::HashMap;

fn main() {
    let mut contador = HashMap::new();

    contador.insert("HTTP", 443);

    contador.entry("HTTP").or_insert(443);
    contador.entry("SSH").or_insert(22);

    println!("{contador:?}");
}
