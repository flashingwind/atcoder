use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    for (i, &c) in s.iter().enumerate() {
        if i % 2 == 0 && c.is_ascii_uppercase() || i % 2 != 0 && c.is_ascii_lowercase() {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
