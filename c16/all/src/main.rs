fn main() {
    let puertos = [80, 443, 8080];

    let todos_mayores = puertos.iter().all(|puerto| *puerto > 0);

    println!("{}", todos_mayores);
}
