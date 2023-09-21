use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    };
    let mut t = VecDeque::new();
    let mut is_rev = false;
    for c in s.iter() {
        if *c == 'R' {
            is_rev = !is_rev;
        } else {
            if is_rev {
                t.push_front(*c);
            } else {
                t.push_back(*c);
            }
        }
        // println!("{}: {:?}", if is_rev { "R" } else { "" }, t);
    }
    if !t.is_empty() {
        let mut is_changed = true;
        // println!("{:?}", t);
        while is_changed {
            is_changed = false;
            let mut t2 = VecDeque::new();
            while let Some(c1) = t.pop_front() {
                if let Some(&c2) = t2.back() {
                    if c2 == c1 {
                        t2.pop_back();
                        is_changed = true;
                    } else {
                        t2.push_back(c1);
                    }
                } else {
                    t2.push_back(c1);
                }
                // println!("{c1}: {:?}<-{:?}", t2, t);
            }
            t = t2.to_owned();
        }
        if is_rev {
            println!("{}", t.iter().rev().collect::<String>());
        } else {
            println!("{}", t.iter().collect::<String>());
        }
    }
}
