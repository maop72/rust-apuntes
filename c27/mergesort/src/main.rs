fn mezclar(izq: &[i32], der: &[i32]) -> Vec<i32> {
    let mut resultado = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < izq.len() && j < der.len() {
        if izq[i] <= der[j] {
            resultado.push(izq[i]);
            i += 1;
        } else {
            resultado.push(der[j]);
            j += 1;
        }
    }

    // Añadimos los elementos que queden en la secuencia izquierda.
    for &x in &izq[i..] {
        resultado.push(x);
    }

    // Añadimos los elementos que queden en la secuencia derecha.
    for &x in &der[j..] {
        resultado.push(x);
    }

    resultado
}

fn mergesort(datos: &[i32]) -> Vec<i32> {
    if datos.len() <= 1 {
        return datos.to_vec();
    }

    let mitad = datos.len() / 2;

    let izq = mergesort(&datos[..mitad]);
    let der = mergesort(&datos[mitad..]);

    mezclar(&izq, &der)
}

fn main() {
    let datos = vec![5, 2, 8, 1, 7, 3];
    println!("Original: {:?}", datos);

    let ordenados = mergesort(&datos);
    println!("Ordenados: {:?}", ordenados);
}
