use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize;n],
    };
    let mut cnt = vec![0; n];
    for i in 0..n {
        for j in 0..3 {
            let k = (n + p[i] - 1 - i + j) % n;
            cnt[k] += 1;
        }
    }
    let mut max = 0;
    for i in 0..n {
        max = cnt[i].max(max);
    }
    println!("{}", max)
}
