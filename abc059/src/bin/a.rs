use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars;3],
    };
    for w in s.iter() {
        print!("{}", w[0].to_uppercase());
    }
    println!();
}
