use proconio::input;

fn main() {
    input! {
        n: f64,
    };
    println!("{}", (n / 5.0).round() as i64 * 5);
}
