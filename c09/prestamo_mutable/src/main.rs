fn main() {
    let mut s = String::from("Hola");

    let prestamo1 = &mut s;
    //s.push('!'); // Sería ilegal: ya hay un préstamo mutable activo
    prestamo1.push('!');

    println!("{}", prestamo1);
    println!("{}", s);
}
