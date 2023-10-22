use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let n = s.len();
    if s[n - 2] == 'e' && s[n - 1] == 'r' {
        println!("er");
    } else {
        println!("ist");
    }
}
