use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        num_q: usize,
    };
    let mut s = VecDeque::new();
    s.push_front('1');
    let mut sum: u64 = 0;
    let modulo = 998244353;
    for _ in 0..num_q {
        input! {
            op: u32,
        };
        if op == 1 {
            input! {
                x: Chars,
            };
            s.extend(x.iter());
            for c in x.iter() {
                sum *= 10;
                sum %= modulo;
                sum += c.to_digit(10).unwrap() as u64;
                sum %= modulo;
            }
        } else if op == 2 {
            println!("s={:?}", s);
            let c = s.pop_front().unwrap().to_digit(10).unwrap() as u64;
            let d = 10u64.pow(s.len() as u32 - 2);
            println!("s={:?}", s);
            println!("{sum}-{c}x{d}:{}", (c * d) % modulo);
            sum -= (c * d) % modulo;
        } else if op == 3 {
            println!("{}", sum);
        }
    }
}
