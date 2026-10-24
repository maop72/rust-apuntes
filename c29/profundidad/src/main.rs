fn busca_profundidad(grafo: &[Vec<usize>], nodo: usize, visitado: &mut [bool]) {
    visitado[nodo] = true;
    print!("{} ", (b'A' + nodo as u8) as char);

    for &vecino in &grafo[nodo] {
        if !visitado[vecino] {
            busca_profundidad(grafo, vecino, visitado);
        }
    }
}

fn main() {
    // A = 0, B = 1, C = 2, ..., H = 7
    let grafo = vec![
        vec![1, 2],       // A: B, C
        vec![0, 3, 4],    // B: A, D, E
        vec![0, 5, 6],    // C: A, F, G
        vec![1, 7],       // D: B, H
        vec![1, 7],       // E: B, H
        vec![2, 7],       // F: C, H
        vec![2],          // G: C
        vec![3, 4, 5],    // H: D, E, F
    ];

    let mut visitado = vec![false; grafo.len()];

    println!("Recorrido en profundidad:");
    busca_profundidad(&grafo, 0, &mut visitado);
    println!();
}
