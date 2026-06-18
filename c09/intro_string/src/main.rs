fn main() {
    let mut s = String::from("¡Hola");
    s.push_str(", mundo");  // Añade cadena (se delimita con comilla doble)
    s.push('!');           // Añade carácter (se delimita con comilla simple)
    println!("{}", s);
}
