use std::f64::consts::PI;

use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        mut d: f64,
    };
    d %= 360.0;
    // if 90 < d {
    //     d = 180 - d;
    // }
    let rad = d * 2. * PI / 360.;
    let x = a * rad.cos() - b * rad.sin();
    let y = a * rad.sin() + b * rad.cos();
    println!("{} {}", x, y);
}
