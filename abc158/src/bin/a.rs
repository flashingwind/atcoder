use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    for (i, _) in s.iter().enumerate().skip(1) {
        if s[i - 1] != s[i] {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
