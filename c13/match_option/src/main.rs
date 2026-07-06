fn main() {
    let numero = Some(10);

    match numero {
        Some(n) => println!("Valor: {}", n),
        None => println!("No hay ningún valor"),
    }
}
