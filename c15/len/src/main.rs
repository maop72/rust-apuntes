fn main() {
    let s1 = String::from("Hello!");
    println!("Cadena: {}", s1);
    println!("Longitud (bytes): {}", s1.len());
    println!("Número de caracteres: {}\n", s1.chars().count());

    let s2 = String::from("¡Hola!");
    println!("Cadena: {}", s2);
    println!("Longitud (bytes): {}", s2.len());
    println!("Número de caracteres: {}", s2.chars().count());
}
