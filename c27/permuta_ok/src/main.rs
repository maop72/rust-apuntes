fn main() {
    let mut a = 10;
    let mut b = 20;

    let aux = a;
    a = b;
    b = aux;

    println!("a = {}, b = {}", a, b); 
}
