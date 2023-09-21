use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut w: Chars,
    };
    let mut cnt: HashMap<char, i32> = HashMap::new();
    for c in w.iter() {
        *cnt.entry(*c).or_insert(0) += 1;
    }
    for (_, count) in cnt.iter() {
        if count % 2 != 0 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
