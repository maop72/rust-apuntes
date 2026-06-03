fn main() {
    let persona = ("Ana", 25, true);

    println!("Contenido de la tupla:");
    println!("\t{:?}", persona);

    println!("Elementos de la tupla:");
    println!("\tNombre: {}", persona.0);
    println!("\tEdad: {}", persona.1);
    println!("\tActivo: {}", persona.2);
}
