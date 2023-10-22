use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64;n],
    };
    let m = 10u64.pow(9) + 7;
    let mut ans = 0;
    let mut sum = vec![0; n];
    sum[n - 1] = *a.last().unwrap();
    for i in (0..=n - 2).rev() {
        sum[i] = sum[i + 1] % m + a[i] % m;
        ans += (a[i] % m * sum[i + 1] % m) % m;
        // println!("{i}: {}x{}", a[i], sum[i + 1])
    }
    // println!("{:?}", sum);
    println!("{}", ans % m);
}
