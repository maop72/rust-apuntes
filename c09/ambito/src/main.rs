fn main() {
    {
        let s = String::from("Hola");
        println!("{}", s);
    }
    println!("{}", s); // ¡ERROR! aquí 's' ya no existe
}
