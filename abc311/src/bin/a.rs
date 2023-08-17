use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let mut cnt = HashMap::new();
    cnt.entry('A').or_insert(0);
    cnt.entry('B').or_insert(0);
    cnt.entry('C').or_insert(0);
    for i in 0..n {
        *cnt.entry(s[i]).or_insert(0) += 1;
        let mut is_comp = true;
        for e in cnt.iter() {
            if *e.1 == 0 {
                is_comp = false;
                break;
            }
        }
        if is_comp {
            println!("{}", i + 1);
            break;
        }
    }
}
