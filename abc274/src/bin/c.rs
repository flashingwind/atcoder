use std::vec;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize;n],
    };
    a.insert(0, 0);
    let mut gen = vec![0; a.len() * 2 + 1];
    for i in 1..=n {
        gen[2 * i] = gen[a[i]] + 1;
        gen[2 * i + 1] = gen[a[i]] + 1;
    }
    for i in 1..=2 * n + 1 {
        println!("{}", gen[i]);
    }
}
