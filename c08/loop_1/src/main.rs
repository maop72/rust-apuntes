fn main() {
    let mut contador = 0;

    loop {
        contador += 1;
        print!("hola ");

        if contador == 5 {
            break;
        }
    }
    println!("Fin del bucle.");
}

