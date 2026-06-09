
fn dividir(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}

fn main() {
    let (cociente, resto) = dividir(17, 5);

    println!("Cociente: {}", cociente);
    println!("Resto: {}", resto);
}
