use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let s2 = "CODEFESTIVAL2016";
    let mut cnt = 0;
    for (i, c) in s2.chars().enumerate() {
        if s[i] != c {
            cnt += 1;
        }
    }
    println!("{cnt}");
}
