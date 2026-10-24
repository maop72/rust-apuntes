use std::collections::VecDeque;

fn busca_anchura(grafo: &[Vec<usize>], inicio: usize) {
    let mut visitado = vec![false; grafo.len()];
    let mut cola = VecDeque::new();

    visitado[inicio] = true;
    cola.push_back(inicio);

    while let Some(nodo) = cola.pop_front() {
        print!("{} ", (b'A' + nodo as u8) as char);

        for &vecino in &grafo[nodo] {
            if !visitado[vecino] {
                visitado[vecino] = true;
                cola.push_back(vecino);
            }
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

    println!("Recorrido en anchura:");
    busca_anchura(&grafo, 0);
    println!();
}
