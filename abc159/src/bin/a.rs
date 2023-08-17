use proconio::input;

fn main() {
    input! {
        n: u32,
        m: u32,
    };
    let mut sum = 0;
    if 2 <= n {
        sum += n * (n - 1) / 2;
    }
    if 2 <= m {
        sum += m * (m - 1) / 2;
    }
    println!("{}", sum);
}
