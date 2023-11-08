use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };
    for i in 0..s.len() / 2 {
        let tmp = s[2 * i + 1];
        s[2 * i + 1] = s[2 * i];
        s[2 * i] = tmp;
    }
    for c in s.iter() {
        print!("{c}");
    }
    println!();
}
