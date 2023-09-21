use std::collections::HashSet;

use proconio::input;
use rustc_hash::FxHashSet;

fn main() {
    input! {
        num_train: usize,
        num_q: usize,
    };
    let mut edges = FxHashSet::from(HashSet::with_capacity(num_train));
    let mut index = vec![Vec::new(); n + 1];
    for _ in 0..num_q {
        input! {
            t: u32,
        };
        if t == 1 {
            input! {
                x: usize,
                y: usize,
            };
            let n = (x - 1, y - 1);
            edges.insert(&n);
            index[x].push(&n);
        } else if t == 2 {
            input! {
                x: usize,
                y: usize,
            };
            edges.remove(&(x - 1, y - 1));
        } else if t == 3 {
            let mut tree: UnionFind<usize> = UnionFind::new(n);
            input! {
                _x: usize,
            };
        }
    }
}
