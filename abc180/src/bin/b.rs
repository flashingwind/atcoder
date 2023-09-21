use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [f64;n],
    };
    let mut d1 = 0.0;
    let mut d2 = 0.0;
    let mut d3 = 0.0;
    for i in 0..n {
        d1 += x[i].abs();
        d2 += x[i] * x[i];
        if d3 < x[i].abs() {
            d3 = x[i].abs();
        }
    }
    println!("{}", d1);
    println!("{}", d2.sqrt());
    println!("{}", d3);
}
