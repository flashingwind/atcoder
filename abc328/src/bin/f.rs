use petgraph::prelude::UnGraph;
use proconio::{input, marker::Usize1};
use rustc_hash::FxHashMap;

fn main() {
    input! {
        n: usize,
        num_q: usize,
        q: [(Usize1,Usize1,u32);num_q],
    };
    let mut nodes = FxHashMap::default();
    let mut g: petgraph::Graph<(), (), petgraph::Undirected> = UnGraph::new_undirected();
    for i in 0..q {
        if let Some(n) = nodes.get_mut(&q[i].0) {
            if let Some(ni) = n {
            } else {
                nodes.insert(q[i].0, Some(g.add_node(())));
            }
            if let Some(ni) = n {
            } else {
                nodes.insert(q[i].0, Some(g.add_node(())));
            }
        }
    }
}
