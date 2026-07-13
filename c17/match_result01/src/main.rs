use std::num::ParseIntError;

fn obtiene_numero(cadena: &str) -> Result<i32, ParseIntError> {
    let numero = match cadena.parse::<i32>() {
        Ok(valor) => valor,
        Err(error) => return Err(error),
    };
    Ok(numero)
}

fn main() {
    println!("{:?}", obtiene_numero("125"));
    println!("{:?}", obtiene_numero("hola"));
}
