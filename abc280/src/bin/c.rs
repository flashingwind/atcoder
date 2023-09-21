use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    for (i, c) in s.iter().enumerate() {
        if *c != t[i] {
            println!("{}", i + 1);
            return;
        }
    }
    println!("{}", t.len());
}
