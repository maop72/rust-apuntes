fn quicksort(datos: &[i32]) -> Vec<i32> {
    if datos.len() <= 1 {
        return datos.to_vec();
    }

    let indice_pivote = datos.len() / 2;
    let pivote = datos[indice_pivote];

    let mut izquierda = Vec::new();
    let mut derecha = Vec::new();

    for (i, &x) in datos.iter().enumerate() {
        if i == indice_pivote {
            continue;
        }

        if x < pivote {
            izquierda.push(x);
        } else {
            derecha.push(x);
        }
    }

    let izquierda = quicksort(&izquierda);
    let derecha = quicksort(&derecha);

    let mut resultado = izquierda;
    resultado.push(pivote);
    resultado.extend(derecha);

    resultado
}

fn main() {
    let datos = [5, 2, 9, 7, 1, 8, 3];

    let ordenados = quicksort(&datos);
    println!("{:?}", ordenados);
}
