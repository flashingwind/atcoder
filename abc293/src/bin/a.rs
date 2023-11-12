use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };
    for i in 0..s.len() / 2 {
        (s[i * 2], s[2 * i + 1]) = (s[i * 2 + 1], s[2 * i]);
    }
    for c in &s {
        print!("{c}");
    }
    println!();
}
