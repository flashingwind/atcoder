use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };

    if t.len() < s.len() {
        println!("No");
        return;
    }
    for (i, &c) in s.iter().enumerate() {
        if t[i] != c {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
