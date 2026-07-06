fn main() {
    let mut pila = vec![10, 20, 30];

    while let Some(valor) = pila.pop() {
        println!("{valor}");
    }
}
