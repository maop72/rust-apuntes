fn selection_sort(datos: &mut [i32]) {
    for i in 0..datos.len() {
        let mut minimo = i;

        for j in i + 1..datos.len() {
            if datos[j] < datos[minimo] {
                minimo = j;
            }
        }

        datos.swap(i, minimo);
    }
}

fn main() {
    let mut numeros = [7, 3, 5, 2, 8, 3];
    println!("Antes:   {:?}", numeros);

    selection_sort(&mut numeros);
    println!("Después: {:?}", numeros);
}
