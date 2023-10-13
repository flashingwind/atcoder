use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize,u64);n],
    };
    let mut dp = vec![vec![0u64; w + wv.iter().max().unwrap().0]; n + 1];
    for i in 0..n {
        for sum_w in 0..=w {
            if wv[i].0 <= sum_w {
                dp[i + 1][sum_w] = dp[i + 1][sum_w].max(dp[i][sum_w - wv[i].0] + wv[i].1);
            }
            dp[i + 1][sum_w] = dp[i + 1][sum_w].max(dp[i][sum_w]);
        }
    }
    // println!("{:?}", dp);
    println!("{}", dp[n][w]);
}
