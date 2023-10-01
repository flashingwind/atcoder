use std::{collections::HashSet, iter::FromIterator};

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: [String;n],
    };
    for i in 0..n {
        let mut r = String::new();
        //r.reverse();
        for c in s[i].chars().rev() {
            r += c.to_string().as_str();
            // println!("r={}", r);
        }
        if s[i] < r {
            s[i] = r.to_owned();
        }
    }
    let set: HashSet<_> = s.iter().collect();
    let mut s_unique = Vec::from_iter(set);
    s_unique.sort();
    // println!("s_unique={:?}", s_unique);
    println!("{}", s_unique.len());
}
