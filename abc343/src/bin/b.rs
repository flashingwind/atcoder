use itertools::Itertools;
use petgraph::graph::UnGraph;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize;n];n],
    };
    let mut g = UnGraph::new_undirected();
    let mut nodes = Vec::new();
    for _ in 0..n {
        nodes.push(g.add_node(()));
    }
    for i in 0..n {
        for j in 0..n {
            if a[i][j] == 1 {
                g.add_edge(nodes[i], nodes[j], ());
            }
        }
    }
    for i in 0..n {
        for j in g.neighbors(nodes[i]).sorted().unique() {
            print!(" {}", j.index() + 1);
        }
        println!();
    }
}
