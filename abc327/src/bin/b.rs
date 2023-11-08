use proconio::input;
use std::u128;

fn main() {
    input! {
        b: u128,
    };
    for a in 1u128..=16 {
        if let Some(aa) = a.checked_pow(a as u32) {
            // println!("cache[{a}]={aa};");
            if aa == b {
                println!("{}", a);
                return;
            }
        } else {
            // println!("{a}^{a}=break");
            break;
        }
    }
    println!("-1");
}
