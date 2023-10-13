use proconio::input;

fn main() {
    input! {
        n: usize,
        w: u64,
        wv: [(u64,usize);n],
    };
    let v_sum_max: usize = wv.iter().map(|&v| v.1).sum();
    let mut dp = vec![vec![u64::MAX - 10u64.pow(9); v_sum_max + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        for v_sum in 0..=v_sum_max {
            if wv[i].1 <= v_sum {
                dp[i + 1][v_sum] = dp[i + 1][v_sum].min(dp[i][v_sum - wv[i].1] + wv[i].0);
            }
            dp[i + 1][v_sum] = dp[i + 1][v_sum].min(dp[i][v_sum]);
        }
    }
    // println!(
    //     "{:?}",
    //     dp[n][..=v_sum_max]
    //         .iter()
    //         .filter(|ww| **ww <= w)
    //         .collect::<Vec<_>>()
    // );
    let mut max = 0;
    for (vi, wi) in dp[n].iter().enumerate() {
        if *wi <= w {
            max = max.max(vi);
        }
    }
    println!("{}", max);
}
