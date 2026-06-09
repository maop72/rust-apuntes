fn mostrar_entero(i: i32) {
    println!("{}", i);
}

fn mostrar_cadena(s: String) {
    println!("{}", s);
}

fn main() {
    let i = 7;
    let s = String::from("hola");

    mostrar_entero(i);
    mostrar_cadena(s);

    println!("{}", i); // Correcto
    println!("{}", s); // ¡ERROR!  's' ha perdido la propiedad
}
