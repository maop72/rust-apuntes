fn modificar_cadena(s: &mut String) {
    s.push('!');
    println!("Cadena prestada: {}", s);
}

fn main() {
    let mut s = String::from("Hola");
    modificar_cadena(&mut s);

    // Recuperamos la cadena prestada, podemos modficarla
    s.push('!');

    println!("Cadena original: {}", s);
}
