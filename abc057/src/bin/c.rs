use num::integer::Roots;
use proconio::input;

fn main() {
    input! {
        n: u64,
    };
    let mut min = 11;
    for a in 1..=n.sqrt() + 1 {
        let b = n / a;
        if a * b == n {
            let f = a.to_string().len().max(b.to_string().len());
            if f < min {
                min = f;
                // println!("{a}x{b}: {f}");
            }
        }
    }
    println!("{}", min);
}
