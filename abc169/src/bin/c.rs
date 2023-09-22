use proconio::{input, marker::Chars};

fn main() {
    input! {
        a: u64,
        b_chars: Chars,
    }
    let mut b = 0u64;
    let mut digits_cnt = 0;
    let mut is_float = false;
    for c in b_chars.iter() {
        if *c == '.' {
            is_float = true;
        } else {
            b *= 10;
            b += c.to_digit(10).unwrap() as u64;
            if is_float {
                digits_cnt += 1;
            }
        }
    }
    println!("{}", (a * b) / 10u64.pow(digits_cnt));
}
