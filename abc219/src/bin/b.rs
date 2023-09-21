use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [String;3],
        ts: Chars,
    };
    for t in ts.iter() {
        print!("{}", s[t.to_digit(10).unwrap() as usize - 1]);
    }
    println!();
}
