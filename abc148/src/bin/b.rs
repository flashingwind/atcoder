use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    };
    for i in 0..s.len() {
        print!("{}{}", s[i], t[i]);
    }
    println!()
}
