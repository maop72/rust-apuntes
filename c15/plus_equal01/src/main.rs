fn main() {
    let mut s = String::from("¡Hola");

    s.push_str(", mundo"); // Añade el &str al string.
    s += "! "; // De nuevo, añade el &str al string. 

    println!("{}", s);
}
