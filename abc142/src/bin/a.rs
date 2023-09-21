use proconio::input;

fn main() {
    input! {
        n: f64,
    };
    let mut cnt = (n / 2.).floor() * 2.;
    println!("{}", cnt / n);
}
