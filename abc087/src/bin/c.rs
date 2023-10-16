use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[u32;n];2],
    };
    let mut dp = vec![vec![0u32; n]; 2];
    for i in 0..2 {
        for j in 0..n {
            if i == 0 && j == 0 {
                dp[i][j] = a[i][j];
            } else if i == 0 {
                dp[i][j] = a[i][j] + dp[i][j - 1];
            } else if j == 0 {
                dp[i][j] = a[i][j] + dp[i - 1][j];
            } else {
                dp[i][j] = a[i][j] + dp[i - 1][j].max(dp[i][j - 1]);
                // println!("{} {}", dp[i - 1][j - 1], dp[i - 1][j]);
                // println!("{} {}", dp[i][j - 1], dp[i][j]);
            }
            // println!("-> {i},{j}: {}: ", dp[i][j]);
        }
    }
    println!("{}", dp.last().unwrap().last().unwrap());
}
