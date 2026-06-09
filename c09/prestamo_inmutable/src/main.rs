fn main() {
    let s = String::from("Hola");

    let prestamo1 = &s;
    let prestamo2 = &s;
    let prestamo3 = &s;

    println!("{}", s);
    println!("{}", prestamo1);
    println!("{}", prestamo2);
    println!("{}", prestamo3);
}
