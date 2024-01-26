use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut sets = vec![vec![HashSet::new(); 10]; n + 1];
    for i in 0..n {
        sets[1][i].insert(i);
        for j in 0..10 {
            for k in sets[1+j][i]
        }
    }
}
