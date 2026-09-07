fn main() {
    let cadena = String::from("Hola");
    let referencia = &cadena;

    println!("{}", cadena);       // Una cadena
    println!("{}", &cadena);      // Una referencia a cadena

    println!("{}", referencia);   // Una referencia a cadena
    println!("{}", *referencia);  // Una cadena
}
