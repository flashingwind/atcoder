use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut n = 0;
    for c in s.iter() {
        if c.is_ascii_digit() {
            n *= 10;
            n += c.to_digit(10).unwrap();
        } else {
            println!("error");
            return;
        }
    }
    println!("{}", n * 2);
}
