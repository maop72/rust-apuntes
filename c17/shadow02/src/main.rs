fn main() {
    let dato = "123";
    let dato = dato.parse::<i32>().unwrap();
    let dato = dato * 2;

    println!("{}", dato);
}
