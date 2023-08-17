use std::vec;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize;n];m],
    };
    // println!("{:?}", a);
    let mut p = vec![vec![0; n + 1]; n + 1];
    for i in 0..m {
        for j in 1..n {
            p[a[i][j - 1]][a[i][j]] = 1;
            p[a[i][j]][a[i][j - 1]] = 1;
        }
    }
    let mut sum = 0;
    for i in 1..=n {
        for j in i + 1..=n {
            if p[i][j] == 0 {
                sum += 1;
                // println!("{i}/{j}={}", p[i][j]);
            }
        }
    }
    println!("{}", sum);
}
