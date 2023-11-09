use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    };
    let mut cnt = vec![vec![0u128; k + m + 1]; n + 1];
    cnt[0][0] = 1;
    for i in 0..n {
        for sum in 0..=k {
            for nxt in 1..=m {
                if sum + nxt <= k {
                    cnt[i + 1][sum + nxt] += cnt[i][sum];
                }
            }
        }
    }
    let mut ans = 0;
    for sum in n..k + 1 {
        ans += cnt[n][sum];
        ans %= 998244353;
    }
    println!("{}", ans % 998244353);
}
