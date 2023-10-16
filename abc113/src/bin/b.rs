use proconio::input;

fn main() {
    input! {
        n: usize,
        t: f64,
        a: f64,
        height: [f64;n],
    };
    let mut min_diff = std::f64::MAX;
    let mut min_diff_i = 0;
    for (i, x) in height.iter().enumerate() {
        let diff = (a - (t - *x * 0.006)).abs();
        // println!("avg={}", (t - *x * 0.006));
        if diff < min_diff {
            min_diff = diff;
            min_diff_i = i + 1;
        }
    }
    println!("{}", min_diff_i);
}
