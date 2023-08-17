use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [f64;n],
    };
    let mut sum = 0.;
    for i in 0..n {
        sum += 1.0 / a[i];
    }
    println!("{}", 1.0 / sum);
}
