use std::collections::HashSet;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut cache = HashSet::new();
    let max = (n as f64).sqrt() as usize;
    for i in 1..=max {
        if n % i == 0 {
            cache.insert(i);
            cache.insert(n / i);
        }
    }
    for nn in cache.iter().sorted() {
        println! {"{}",nn};
    }
}
