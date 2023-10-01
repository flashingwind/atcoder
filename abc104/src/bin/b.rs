use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    };
    if s[0] != 'A' {
        println!("WA");
        return;
    }
    let mut cnt_c = 0;
    for i in 1..s.len() {
        if 2 <= i && i <= s.len() - 2 && s[i] == 'C' {
            cnt_c += 1;
        } else if !s[i].is_ascii_lowercase() {
            println!("WA");
            return;
        }
    }
    if cnt_c != 1 {
        println!("WA");
        return;
    }
    println!("AC");
}
