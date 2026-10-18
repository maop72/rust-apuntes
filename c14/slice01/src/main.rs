fn mostrar(numeros: &[i32]) {
    println!("{numeros:?}");
}

fn main() {
    let numeros = vec![0, 10, 20, 30, 40, 50];

    mostrar(&numeros);
    mostrar(&numeros[1..4]);
}
