use proconio::input;

fn main() {
    input! {
        n: usize,
        d: u64,
        a: [u64;n],
    };
    let mut cnt = vec![vec![0; n]; n];
    for i in 0..n {
        let max_cnt = 0;
        for ii in 0..=i {}
        for j in i + 1..n {
            if a[i].abs_diff(a[j]) <= d {
                cnt[i][j] = cnt[i][j];
            }
        }
    }
}
