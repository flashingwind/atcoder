use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };
    s.sort();
    println!("{}", s.iter().map(|v| v.to_string()).join(""));
}