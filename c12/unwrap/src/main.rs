fn main() {
    let x = Some(42);
    println!("{:?}", x); // Enumerado completo
    println!("{}", x.unwrap()); // Valor Some

    let y: Option<i32> = None;
    println!("{}", y.unwrap()); // Panic
}
