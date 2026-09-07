fn main() {
    let puertos = [80, 443, 8080];
    let mut iterador = puertos.iter();

    println!("{:?}", iterador.next());
    println!("{:?}", iterador.next());
    println!("{:?}", iterador.next());
    println!("{:?}", iterador.next());  // None
    println!("{:?}", iterador.next());  // None
}
