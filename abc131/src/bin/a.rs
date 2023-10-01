use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    for i in 1..s.len() {
        if s[i - 1] == s[i] {
            println!("Bad");
            return;
        }
    }
    println!("Good");
}
