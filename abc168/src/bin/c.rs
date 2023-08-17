use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64,
    };
    let a_in_min = h / 12.0 * 60.0 + m * 1.0 / 12.0; //時針の補正
    let b_in_min = m;
    // println!("a_in_min={a_in_min}");
    let mut rad = (b_in_min - a_in_min).abs();
    // // println!("diff min={}", rad);
    if 30.0 < rad {
        rad = 60.0 - rad;
    }
    // println!("diff min={}", rad);
    // println!("diff deg={}", rad * 6.0);
    rad *= std::f64::consts::PI * 2. / 60.0;
    println!(
        "{:.20}",
        (a.powi(2) + b.powi(2) - 2.0 * a * b * f64::cos(rad)).sqrt()
    );
}
