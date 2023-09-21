use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        a: [u32;n],
    };
    let mut num_candies = vec![k / n; n];
    k = k % n;
    for (i, _) in a.iter().enumerate().sorted_by(|a, b| a.1.cmp(&b.1)) {
        if k != 0 {
            num_candies[i] += 1;
            k -= 1;
        } else {
            break;
        }
    }
    for n in num_candies.iter() {
        println!("{}", n);
    }
}
