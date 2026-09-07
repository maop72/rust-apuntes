fn main() {
    let numero = 443;
    let referencia = &numero;
    println!("El número es {}", referencia);

    let doble = *referencia * 2;  // Desreferencia explícita. Bien. 
    println!("El doble es {}", doble);

    let doble = referencia * 2;   // El compilador desreferencia. Bien también.
    println!("El doble es {}", doble);
}
