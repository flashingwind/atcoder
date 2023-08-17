use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    print!("0");
    for c in s[0..3].iter() {
        print!("{}", c);
    }
    println!();
}
