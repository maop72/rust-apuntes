fn main() {
    let mut contador = 0;

    let resultado = loop {
        contador += 1;

        if contador == 5 {
            break contador;
        }
    };

    println!("{}",resultado);
}
