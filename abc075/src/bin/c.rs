use petgraph::algo::connected_components;
use petgraph::prelude::*;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        len_es: usize,
        es: [(Usize1,Usize1);len_es],
    };
    let mut g = UnGraph::<(), (), usize>::from_edges(es);
    let mut cnt = 0;
    let g2 = g.to_owned();
    let edges = g2
        .edge_indices()
        .map(|v| (v, g.edge_endpoints(v).unwrap()))
        .collect::<Vec<_>>();
    for &(e, (a, b)) in edges.iter() {
        let cc = connected_components(&g);
        g.remove_edge(e);

        if cc != connected_components(&g) {
            cnt += 1;
        }
        g = g2.to_owned();
    }
    println!("{cnt}");
}
