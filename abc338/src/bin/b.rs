use proconio::{input, marker::Chars};
use rustc_hash::FxHashMap;

fn main() {
    input! {
        mut s: Chars,
    };
    let mut h = FxHashMap::default();
    let mut max_c = 'X';
    s.sort();
    for &c in s.iter() {
        *h.entry(c).or_insert(0) += 1;
        if *h.entry(max_c).or_insert(0) < *h.entry(c).or_insert(0) {
            max_c = c;
        }
    }
    println!("{}", max_c);
}
