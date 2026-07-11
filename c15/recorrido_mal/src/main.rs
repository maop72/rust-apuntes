fn main() {
    let s = String::from("¡Hola!");

    for i in 0..s.len() {
        print!("{}", s[i]);  // ¡Error! En Rust no se puede usar [i] con cadenas
    }
    println!();
}
