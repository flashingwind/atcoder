use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32;n-1],
    };
    let mut sum = 0;
    for i in 0..n - 1 {
        sum += a[i];
    }
    println!("{}", -sum);
}
