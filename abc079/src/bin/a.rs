use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut is_ok = true;
    for i in 1..3 {
        if s[0] != s[i] {
            is_ok = false;
        }
    }
    for i in 2..4 {
        if s[1] != s[i] && !is_ok {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
