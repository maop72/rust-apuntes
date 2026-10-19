fn modificar(datos: &mut [i32]) {
    datos[0] = 10;
}

fn main() {
    let mut numeros = [3, 5, 7];

    modificar(&mut numeros);
    println!("{numeros:?}");
}
