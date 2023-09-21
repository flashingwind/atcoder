use std::vec;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut p: [usize;n],
    };
    p.insert(0, 0);
    let mut q = vec![0usize; n + 1];
    for i in 1..=n {
        q[p[i]] = i;
    }
    println!("{}", q.iter().skip(1).map(|c| c.to_string()).join(" "));
}
