use proconio::input;

fn main() {
    input! {
        n: usize,
        d: f64,
        h: f64,
        t: [(f64,f64);n],
    };
    let mut b = vec![0f64; n];
    let mut b_max = 0f64;
    for i in 0..n {
        b[i] = (t[i].0 * h - d * t[i].1) / (t[i].0 - d);
        if b_max < b[i] {
            b_max = b[i];
        }
    }
    println!("{}", b_max);
}
