use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };
    // 6 を 9 に、9 を 6 に変換する。
    s.reverse();
    for c in s.iter_mut() {
        if *c == '6' {
            *c = '9';
        } else if *c == '9' {
            *c = '6';
        }
        print!("{}", c);
    }
    println!();
}
