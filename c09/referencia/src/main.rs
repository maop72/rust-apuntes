fn mostrar_cadena(s: &String) {
    println!("{}", s);
}

fn main() {
    let s = String::from("Hola");
    mostrar_cadena(&s);
    println!("{}", s); // Correcto
}
