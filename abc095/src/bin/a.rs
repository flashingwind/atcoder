use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    };
    let mut cost = 700;
    for c in s.iter() {
        if *c == 'o' {
            cost += 100;
        }
    }
    println!("{}", cost);
}
