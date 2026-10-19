fn insertion_sort(datos: &mut [i32]) {
    for i in 1..datos.len() {
        let mut j = i;

        while j > 0 && datos[j] < datos[j - 1] {
            datos.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn main() {
    let mut numeros = [7, 3, 5, 2, 8, 3];
    println!("Antes:   {:?}", numeros);

    insertion_sort(&mut numeros);
    println!("Después: {:?}", numeros);
}
