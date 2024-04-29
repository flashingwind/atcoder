use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        mut araw: [u64;n],
    };
    let a = araw.iter().sorted().unique().map(|v| *v).collect_vec();
    let mut sum: u64;
    sum = (1 + k) * k / 2;

    // println!("(1 + {k}) * {k} / 2={}", sum);
    // println!("{:?}", a);
    for i in 0..a.len() {
        if a[i] <= k {
            sum -= a[i];
        } else {
            break;
        }
    }
    println!("{}", sum);
}
