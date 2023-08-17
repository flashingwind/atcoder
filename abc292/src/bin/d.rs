use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        num_nodes: usize,
        num_edges: usize,
    };
    let mut g: UnionFind<usize> = UnionFind::new(num_nodes);
    let mut edges = Vec::new();
    for _ in 0..num_edges {
        input! {
            e: (usize,usize),
        };
        edges.push(e.to_owned());
        g.union(e.0 - 1, e.1 - 1);
    }
    // println!("{:?}", n_labels);
    let mut cnt_n = BTreeMap::new();
    let mut cnt_e = BTreeMap::new();
    for i in 0..num_nodes {
        let l = g.find(i);
        *cnt_n.entry(l).or_insert(0usize) += 1;
    }
    for e in edges.iter() {
        let l = g.find(e.0 - 1);
        *cnt_e.entry(l).or_insert(0usize) += 1;
    }

    // println!("cnt_e={:?}", cnt_e);
    // println!("cnt_n={:?}", cnt_n);
    let bind_e = cnt_e.to_owned();
    let itr_e = bind_e.iter();
    let bind_n = cnt_n.to_owned();
    let itr_n = bind_n.iter();

    for (l, _) in itr_e.merge(itr_n) {
        if *cnt_e.entry(*l).or_default() != *cnt_n.entry(*l).or_default() {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
