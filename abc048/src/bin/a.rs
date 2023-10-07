use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: String,
        s: Chars,
        _: String,
    };
    println!("A{}C", s[0]);
}
