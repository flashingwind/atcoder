use proconio::{input, marker::Chars};
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let mut x = n;
    let mut y = n;
    let mut is_checked = BTreeSet::new();
    is_checked.insert((x, y));
    for c in s.iter() {
        if *c == 'R' {
            x += 1;
        } else if *c == 'L' {
            x -= 1;
        } else if *c == 'U' {
            y += 1;
        } else if *c == 'D' {
            y -= 1;
        }
        if is_checked.contains(&(x, y)) {
            println!("Yes");
            return;
        } else {
            is_checked.insert((x, y));
        }
    }
    println!("No");
}
