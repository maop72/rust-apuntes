fn main() {
    let mut a = 10;
    let mut b = 20;

    a = b;
    b = a;  // ¡Mal!

    println!("a = {}, b = {}", a, b); 
}
