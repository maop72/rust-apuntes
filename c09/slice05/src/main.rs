fn main() {
    let mut numeros = [0, 10, 20, 30, 40, 50];

    let parte = &mut numeros[1..4];
    let referencia = &numeros[2]; // ¡ERROR! Préstamos incompatibles

    println!("{referencia}");
    parte[0] = 200;
}
