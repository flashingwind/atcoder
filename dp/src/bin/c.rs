use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        a: [[u32;3];n],
    };
    let idx = BTreeSet::from([0, 1, 2]);
    let mut dp = vec![vec![0; 3]; n];
    dp[0][0] = a[0][0];
    dp[0][1] = a[0][1];
    dp[0][2] = a[0][2];
    for i in 1..n {
        for &j in idx.iter() {
            let another = &idx - &BTreeSet::from([j]);
            // println!("{j}+{:?}", another);
            for &k in another.iter() {
                // println!(
                //     "    [{}][{k}]->[{i}][{j}]: {} or {}",
                //     i - 1,
                //     dp[i][j],
                //     dp[i - 1][k] + a[i][j]
                // );
                dp[i][j] = dp[i][j].max(dp[i - 1][k] + a[i][j]);
            }
        }
    }
    println!("{}", dp[n - 1][0].max(dp[n - 1][1]).max(dp[n - 1][2]))
}
