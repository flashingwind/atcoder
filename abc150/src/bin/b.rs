use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: String,
    };
    println!("{}", s.matches("ABC").count());
}
