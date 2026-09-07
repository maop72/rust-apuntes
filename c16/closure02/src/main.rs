fn main() {
    let describir = |numero| {
        let doble = numero * 2;
        format!("El doble es {}", doble)
    };

    println!("{}", describir(5));
}
