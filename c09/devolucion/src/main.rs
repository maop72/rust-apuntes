fn mostrar_cadena(s: String) -> String {
    println!("{}", s);
    s // La función devuelve la cadena
}

fn main() {
    let mut s = String::from("hola");
    s = mostrar_cadena(s);
    println!("{}", s); //  Correcto
}
