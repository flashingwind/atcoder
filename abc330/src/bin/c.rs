use proconio::input;

fn main() {
    input! {
        d: i128,
    };
    let mut min = i128::MAX;
    for x in 0..=(d as f64).sqrt().ceil() as i128 {
        let c = x * x - d;
        let y = 0i128;
        min = min.min((c + y * y).abs());
        let y = (c as f64 * -1.0).sqrt().floor() as i128;
        min = min.min((c + y * y).abs());
        let y = (c as f64 * -1.0).sqrt().ceil() as i128;
        min = min.min((c + y * y).abs());
        // println!(
        //     "{x},{y}: {}+{}-{}={}",
        //     x * x,
        //     y * y,
        //     d,
        //     (x ^ 2 + y ^ 2).abs_diff(d)
        // );
    }
    println!("{min}");
}
