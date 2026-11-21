use std::convert::TryFrom;

fn main() {
    let x: i32 = 300;
    let y = u8::try_from(x);

    match y {
        Ok(valor) => println!("Valor convertido: {}",valor),
        Err(_) => println!("El valor {} no cabe en un u8",x),
    }
}
