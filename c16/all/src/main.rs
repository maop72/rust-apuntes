fn main() {
    let puertos = [80, 443, 8080];

    let todos_positivos = puertos
        .iter()
        .all(|puerto| *puerto > 0);
    println!("Todos positivos: {}", todos_positivos);
}
