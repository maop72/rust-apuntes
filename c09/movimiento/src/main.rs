fn main() {
    let i1 = 5;
    let i2 = i1;

    println!("{}", i1);
    println!("{}", i2);

    let s1 = String::from("Hola");
    let s2 = s1;

    println!("{}", s2);
    println!("{}", s1); // ¡MAL!  s1 ya no es el propietario de la cadena
}
