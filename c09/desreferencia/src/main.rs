fn duplicar(numero: i32) -> i32 {
    numero * 2
}

fn main() {
    let numero = 443;
    let referencia = &numero;

    //let doble = duplicar(referencia);   // ¡Error!
    let doble = duplicar(*referencia);

    println!("El doble de {} es {}", referencia, doble);
}
