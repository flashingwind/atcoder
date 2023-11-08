use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    println!("{}", s.iter().map(|v| v.to_digit(10).unwrap()).sum::<u32>());
}
