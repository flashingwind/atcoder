use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    if s[0].is_ascii_uppercase() {
        for i in 1..s.len() {
            if s[i].is_ascii_uppercase() {
                println!("No");
                return;
            }
        }
    } else {
        println!("No");
        return;
    }
    println!("Yes");
}
