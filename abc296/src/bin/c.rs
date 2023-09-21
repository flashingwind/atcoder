use std::{collections::BTreeSet, iter::FromIterator};

use proconio::input;

fn main() {
    input! {
        n: usize,
        xx: i32,
        mut a: [i32;n],
    };
    if xx == 0 {
        println!("Yes");
        return;
    }
    // a.sort_unstable();
    let x = xx.abs();
    let set: BTreeSet<i32> = BTreeSet::from_iter(a.iter().cloned());

    for i in 0..n {
        if set.contains(&(a[i] + x)) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
