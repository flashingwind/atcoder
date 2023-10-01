use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    for (i, c) in s.iter().enumerate() {
        if i % 2 == 0 {
            print!("{}", c);
        }
    }
    println!();
}
