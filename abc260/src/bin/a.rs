use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut cnt: HashMap<char, i32> = HashMap::default();
    for &c in s.iter() {
        *cnt.entry(c).or_insert(0) += 1;
    }
    let mut no_output = true;
    for (c, count) in cnt.iter() {
        if *count == 1 {
            println!("{c}");
            no_output = false;
            return;
        }
    }
    if no_output {
        println!("-1");
    }
}
