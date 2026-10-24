use petgraph::graph::UnGraph;
use petgraph::visit::{Dfs, Bfs};

fn main() {
    let mut grafo = UnGraph::<char, ()>::new_undirected();

    let a = grafo.add_node('A');
    let b = grafo.add_node('B');
    let c = grafo.add_node('C');
    let d = grafo.add_node('D');
    let e = grafo.add_node('E');
    let f = grafo.add_node('F');
    let g = grafo.add_node('G');
    let h = grafo.add_node('H');

    grafo.add_edge(a, b, ());
    grafo.add_edge(a, c, ());
    grafo.add_edge(b, d, ());
    grafo.add_edge(b, e, ());
    grafo.add_edge(c, f, ());
    grafo.add_edge(c, g, ());
    grafo.add_edge(d, h, ());
    grafo.add_edge(e, h, ());
    grafo.add_edge(f, h, ());

    println!("BFS, Broad-First Search:");
    let mut bfs = Bfs::new(&grafo, a);
    while let Some(nodo) = bfs.next(&grafo) {
        print!("{} ", grafo[nodo]);
    }
    println!("\n");

    println!("DFS, Depth-First Search:");
    let mut dfs = Dfs::new(&grafo, a);
    while let Some(nodo) = dfs.next(&grafo) {
        print!("{} ", grafo[nodo]);
    }
    println!("\n");
}
