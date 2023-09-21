use proconio::input;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    input! {
        n: usize,
        a: [u32;n],
    };
    let set: HashSet<u32> = HashSet::from_iter(a.iter().cloned());
    if set.len() == n {
        println!("YES");
    } else {
        println!("NO");
    }
}
