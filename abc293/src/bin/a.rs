use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };
    let s_len = s.len();
    s.insert(0, '0');
    for i in 1..=s_len / 2 {
        let tmp = s[2 * i - 1];
        s[2 * i - 1] = s[2 * i];
        s[2 * i] = tmp;
    }
    for i in 1..=s_len {
        print!("{}", s[i]);
    }
    println!();
}
