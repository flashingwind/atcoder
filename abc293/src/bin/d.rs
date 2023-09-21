use std::collections::{BTreeMap, BTreeSet};

use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::unionfind::UnionFind;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    // Union Find Treeの初期化
    let mut tree: UnionFind<usize> = UnionFind::new(n);
    let mut loop_rep = BTreeSet::new();
    let mut non_loop_rep = BTreeSet::new();
    for i in 0..m {
        input! {
            n1: Usize1,
            _: char,
            n2: Usize1,
            _: char,
        };
        let flag1: bool = tree.equiv(n1, n2);
        tree.union(n1, n2);
        let flag2: bool = tree.equiv(n1, n2);
        if flag1 && flag2 {
            loop_rep.insert(tree.find(n1));
        }
    }

    for i in 0..n {
        if !loop_rep.contains(&tree.find(i)) {
            non_loop_rep.insert(tree.find(i));
        }
    }
    println!("{} {}", loop_rep.len(), non_loop_rep.len());
}
