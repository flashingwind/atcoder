use proconio::input;

fn main() {
    input! {
        n: f64,
    };
    let x = (n / 1.08).floor();
    // println!("{x} ((n) / 1.08).floor()={}", ((n) / 1.08).floor());
    if (x * 1.08).floor() == n {
        println!("{x}");
    } else if ((x + 1.0) * 1.08).floor() == n {
        println!("{}", x + 1.0);
    } else {
        println!(":(");
    }
}
