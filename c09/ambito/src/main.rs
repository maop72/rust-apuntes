fn main() {
    {
        let s = String::from("Hola");
        println!("{}", s);
    }
    println!("{}", s); // ¡MAL! aquí 's' ya no existe
}
