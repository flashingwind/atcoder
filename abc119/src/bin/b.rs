use proconio::input;

fn main() {
    input! {
        n: usize,
        c: [(f64,String);n],
    };
    let mut sum = 0.;
    let btc = 380000.0f64;
    for i in 0..n {
        if c[i].1 == "JPY" {
            sum += c[i].0;
        } else {
            sum += c[i].0 * btc;
        }
    }
    println!("{}", sum);
}
