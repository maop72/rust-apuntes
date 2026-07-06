fn main() {   
    let mut numeros = Vec::new();

    numeros.push(10);
    numeros.push(20);
    numeros.push(30);

    println!("Número de elementos: {}", numeros.len());
    println!("Primer elemento: {}", numeros[0]);
    println!("Segundo elemento: {}", numeros[1]);

    println!("{:?}", numeros);
}

