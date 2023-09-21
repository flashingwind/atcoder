use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    };
    println!("{:5.3}", (b * 1000. / a).round() / 1000.);
}
