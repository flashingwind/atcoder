use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut cost_even_1 = 0;
    let mut cost_even_0 = 0;
    for (i, c) in s.iter().enumerate() {
        if i % 2 == 0 && *c == '0' || i % 2 == 1 && *c == '1' {
            cost_even_1 += 1;
        } else if i % 2 == 0 && *c == '1' || i % 2 == 1 && *c == '0' {
            cost_even_0 += 1;
        }
    }
    if cost_even_0 < cost_even_1 {
        println!("{}", cost_even_0);
    } else {
        println!("{}", cost_even_1);
    }
}
