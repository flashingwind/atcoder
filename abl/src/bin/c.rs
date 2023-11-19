use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        es: [(Usize1,Usize1);m],
    };
    let mut g = UnionFind::new(n);
    for i in 0..m {
        g.union(es[i].0, es[i].1);
        // println!("{:?}", (es[i].0, es[i].1));
    }
    // println!("{:?}", g.clone().into_labeling());
    let labels = g.into_labeling();
    let comps = labels.iter().unique().collect_vec();
    if comps.len() == 0 {
        println!("0")
    } else {
        println!("{}", comps.len() - 1);
    }
}
