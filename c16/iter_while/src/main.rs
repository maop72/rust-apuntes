fn main() {
    let puertos = [80, 443, 8080];
    let mut iterador = puertos.iter();

    loop {
        match iterador.next() {
            Some(puerto) => println!("Puerto: {}", puerto),
            None => break,
        }
    }
}
