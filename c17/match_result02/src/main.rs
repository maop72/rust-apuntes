use std::num::ParseIntError;

fn obtiene_numero(cadena: &str) -> Result<i32, ParseIntError> {
    let numero = cadena.parse::<i32>()?;
    Ok(numero)
}

fn main() {
    println!("{:?}", obtiene_numero("125"));
    println!("{:?}", obtiene_numero("hola"));
}
