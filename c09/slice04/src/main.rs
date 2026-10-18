fn duplicar(numeros: &mut [i32]) {
    for numero in numeros {
        *numero *= 2;
    }
}

fn main() {
    let mut numeros = [0, 10, 20, 30, 40, 50];

    duplicar(&mut numeros[1..4]);
    println!("{numeros:?}");
}
