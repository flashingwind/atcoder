use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    };
    let mut last_c = n[0].to_digit(10).unwrap();
    for c in n.iter().skip(1) {
        if last_c <= c.to_digit(10).unwrap() {
            println!("No");
            return;
        }
        last_c = c.to_digit(10).unwrap();
    }
    println!("Yes");
}
