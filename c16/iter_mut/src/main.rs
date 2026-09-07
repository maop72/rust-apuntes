fn main() {
    let mut puertos = [80, 443, 8080];
    println!("{:?}", puertos);

    // Iterador explícito
    for puerto in puertos.iter_mut() {
        *puerto += 1;
    }
    println!("{:?}", puertos);

    // Sintaxis simplificada
    for puerto in &mut puertos {
        *puerto += 1;
    }
    println!("{:?}", puertos);
}
