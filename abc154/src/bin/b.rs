use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    for _ in s.iter() {
        print!("x");
    }
    println!();
}
