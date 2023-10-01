use petgraph::algo::connected_components;
use petgraph::Graph;
use proconio::input;
use proconio::marker::Usize1;
use rustc_hash::FxHashMap;
fn main() {
    let mut g = Graph::new_undirected();
    input! {
        num_nodes: usize,
        num_es: usize,
        es: [(Usize1,Usize1);num_es],
    };
    let mut nodes = FxHashMap::default();
    for i in 0..num_nodes {
        // nodes.push(g.add_node(()));
        nodes.insert(i, g.add_node(()));
    }
    let mut edges = FxHashMap::default();
    for &(i, j) in es.iter() {
        edges.entry((i, j)).or_insert(g.add_edge(
            *nodes.get(&i).unwrap(),
            *nodes.get(&j).unwrap(),
            (),
        ));
        // println!("{:?}", g.edge_endpoints(*edges.entry((i, j)).or_default()));
    }
    // println!("{:?}", nodes);
    let mut cnt = 0;
    for (&(i, j), e) in edges.iter_mut() {
        let a = g.edge_endpoints(*e).unwrap().0;
        let b = g.edge_endpoints(*e).unwrap().1;
        g.remove_edge(*e);
        let cc = connected_components(&g);
        if cc == 2 {
            cnt += 1;
        }
        *e = g.add_edge(a, b, ());
        // println!("e: {}-{} {:?}-{:?} cc={cc}", i, j, a, b);
    }
    println!("{cnt}");
}
