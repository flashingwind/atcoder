use petgraph::data::FromElements;
use petgraph::{algo::min_spanning_tree, prelude::UnGraph};
use proconio::input;

fn main() {
    input! {
        w: usize,
        h: usize,
        p: [u64;w],
        q: [u64;h],
    };
    let mut ns: Vec<Vec<Option<_>>> = vec![vec![None; w + 1]; h + 1];
    let mut g = UnGraph::new_undirected();
    for i in 0..=h {
        for j in 0..=w {
            ns[i][j] = Some(g.add_node(()));
        }
    }
    for i in 0..h {
        for j in 0..w {
            if let Some(n00) = ns[i][j] {
                if let Some(n10) = ns[i + 1][j] {
                    if let Some(n01) = ns[i][j + 1] {
                        g.add_edge(n00, n10, p[i]);
                        g.add_edge(n00, n01, q[i]);
                    }
                }
            }
        }
    }
    let mst = UnGraph::<_, _>::from_elements(min_spanning_tree(&g));
    let sum: u64 = mst.raw_edges().iter().map(|v| v.weight).sum();
    println!("{sum}");
}
