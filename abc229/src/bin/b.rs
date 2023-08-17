use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut a: Chars,
        mut b: Chars,
    };
    a.reverse();
    b.reverse();
    for (aa, bb) in a.iter().zip(b.iter()) {
        if 10 <= aa.to_digit(10).unwrap_or(0) + bb.to_digit(10).unwrap_or(0) {
            println!("Hard");
            return;
        }
    }
    println!("Easy");
}
