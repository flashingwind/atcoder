use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    if s.len() < 3 || !s[0].is_alphabetic() {
        println!("No");
        return;
    }
    if s[1].is_ascii_digit() && s[1] == '0' {
        println!("No");
        return;
    }
    //for i in 1..s.len() - 1 {
    for s_i in s.iter().take(s.len() - 1).skip(1) {
        if !s_i.is_ascii_digit() {
            println!("No");
            return;
        }
    }
    if !s.last().unwrap().is_alphabetic() {
        println!("No");
        return;
    }
    println!("Yes");
}
