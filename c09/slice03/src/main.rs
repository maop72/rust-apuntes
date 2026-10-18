fn mostrar(numeros: &[i32]) {
    for numero in numeros {
        print!("{numero} ");
    }
    println!();
}

fn main() {
    let numeros = [0, 10, 20, 30, 40, 50];

    mostrar(&numeros);
    mostrar(&numeros[1..4]);
}
