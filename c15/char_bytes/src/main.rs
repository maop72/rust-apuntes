fn main() {
    let s = String::from("¡Hola!");

    println!("Recorrido por caracteres:");
    for c in s.chars() {
        print!("{}, ", c);
    }
    println!();

    println!("Recorrido por bytes:");
    for b in s.bytes() {
        print!("{}, ", b);
    }
    println!();
}
