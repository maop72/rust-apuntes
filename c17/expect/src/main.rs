fn main() {
    let numero = "hola"
        .parse::<i32>()
        .expect("La cadena debería contener un número entero");

    println!("{numero}");
}
