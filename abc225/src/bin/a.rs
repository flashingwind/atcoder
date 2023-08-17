use itertools::Itertools;
use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {
        mut s: Chars,
    };
    let mut pats = HashSet::new();
    for p in (0..3).permutations(3) {
        let mut ss = String::new();
        for i in p.iter() {
            ss.push(s[*i]);
        }
        pats.insert(ss);
    }
    println!("{}", pats.len());
}
