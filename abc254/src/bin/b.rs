use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut a = vec![vec![0u32; n]; n];
    for i in 0..n {
        a[i][0] = 1;
        for j in 1..=i {
            if j == i {
                a[i][j] = 1;
            } else {
                a[i][j] = a[i - 1][j - 1] + a[i - 1][j];
            }
        }
        println!(
            "{}",
            a[i].iter()
                .filter(|v| **v != 0)
                .map(|v| v.to_string())
                .collect_vec()
                .join(" ")
        );
    }
}
