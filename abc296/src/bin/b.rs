use proconio::{input, marker::Chars};

fn main() {
    for l in (1..=8).rev() {
        input! {
            s: Chars,
        };
        for (i, c) in ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'].iter().enumerate() {
            if s[i] == '*' {
                println!("{}{}", *c, l);
            }
        }
    }
}
