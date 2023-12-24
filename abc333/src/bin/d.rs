use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::input;
use proconio::marker::Usize1;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        edges: [(Usize1,Usize1);n-1],
    };
    let mut g = UnionFind::new(n);
    let mut sons = vec![];
    for e in edges {
        if e.0 != 0 && e.1 != 0 {
            g.union(e.0, e.1);
        } else {
            if e.0 != 0 {
                sons.push(e.1);
            } else {
                sons.push(e.0);
            }
        }
    }
    let mut cnt_per_sons = BTreeMap::new();
    for node in 1..n {
        for s in sons.iter() {
            *cnt_per_sons.entry(g.find(node)).or_insert(0usize) += 1;
        }
    }
    let mut sum = 0;
    for (node, cnt) in cnt_per_sons.iter().sorted_by(|a, b| a.1.cmp(&b.1)).skip(1) {
        sum += cnt;
    }
    println!("{sum}");
}
