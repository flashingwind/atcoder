use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(Usize1,Usize1);n],
    };
    // dp[i][j]=i回目のジャンプのあとで座標jにいることができるか？
    let mut dp = vec![vec![false; x + 1]; n + 1];
    dp[0][0] = true;
    for t in 0..=n {
        for i in 0..=n {
            if dp[t][i] {
                if i + ab[t].0 <= x {
                    dp[t][i + ab[t].0] = true;
                }
                if i + ab[t].1 <= x {
                    dp[t][i + ab[t].1] = true;
                }
            }
        }
    }
    if dp[n][x] {
        println!("Yes");
    } else {
        println!("Yes");
    }
}
