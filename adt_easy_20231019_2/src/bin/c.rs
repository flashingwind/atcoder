use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut is_inc_upper = false;
    let mut is_inc_lower = false;
    for c in s.iter() {
        if c.is_ascii_uppercase() {
            is_inc_upper = true;
        } else {
            is_inc_lower = true;
        }
    }
    let set: HashSet<char> = s.iter().clone().map(|v| *v).collect();
    let mut is_diff = false;
    if set.len() == s.len() {
        is_diff = true;
    }
    if is_diff && is_inc_lower && is_inc_upper {
        println!("Yes");
    } else {
        println!("No");
    }
}
