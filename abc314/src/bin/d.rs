use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
    };
    let mut is_cap = None;
    let mut from_last_cap = Vec::new();
    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
            c: char,
        };
        if t == 1 {
            s[x - 1] = c;
            from_last_cap.push((x - 1, c));
        } else if t == 2 {
            is_cap = Some(false);
            from_last_cap.clear();
        } else if t == 3 {
            is_cap = Some(true);
            from_last_cap.clear();
        }
    }
    if let Some(is_capital) = is_cap {
        if is_capital {
            for i in 0..n {
                s[i] = s[i].to_ascii_uppercase();
            }
        } else {
            for i in 0..n {
                s[i] = s[i].to_ascii_lowercase();
            }
        }
    }
    // println!("{:?}", from_last_cap);
    for (x, c) in from_last_cap.iter() {
        s[*x] = *c;
    }
    println!("{}", s.iter().join(""));
}
