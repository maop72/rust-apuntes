fn dijkstra(
    grafo: &[Vec<(usize, u32)>],
    inicio: usize,
) -> Vec<u32> {
    let n = grafo.len();
    let infinito = u32::MAX;

    let mut distancias = vec![infinito; n];
    let mut procesado = vec![false; n];

    distancias[inicio] = 0;

    for _ in 0..n {
        // Buscar el nodo no procesado con menor distancia.
        let mut actual = None;

        for i in 0..n {
            if !procesado[i]
                && distancias[i] != infinito
                && actual.is_none_or(|j| distancias[i] < distancias[j])
            {
                actual = Some(i);
            }
        }

        // Si no quedan nodos alcanzables, terminamos.
        let Some(u) = actual else {
            break;
        };

        procesado[u] = true;

        // Actualizar las distancias de los vecinos.
        for &(v, peso) in &grafo[u] {
            let nueva_distancia = distancias[u] + peso;

            if nueva_distancia < distancias[v] {
                distancias[v] = nueva_distancia;
            }
        }
    }

    distancias
}

fn main() {
    // A = 0, B = 1, ..., H = 7
    let grafo = vec![
        vec![(1, 4), (2, 2)],             // A
        vec![(0, 4), (3, 5), (4, 2)],    // B
        vec![(0, 2), (5, 3), (6, 8)],    // C
        vec![(1, 5), (7, 3)],             // D
        vec![(1, 2), (7, 4)],             // E
        vec![(2, 3), (7, 2)],             // F
        vec![(2, 8)],                     // G
        vec![(3, 3), (4, 4), (5, 2)],    // H
    ];

    let distancias = dijkstra(&grafo, 0);

    for (i, distancia) in distancias.iter().enumerate() {
        let nodo = (b'A' + i as u8) as char;

        if *distancia == u32::MAX {
            println!("{nodo}: inalcanzable");
        } else {
            println!("{nodo}: {distancia}");
        }
    }
}
