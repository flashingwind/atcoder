use proconio::input;

fn main() {
    input! {
        r: u64,
        x: u64,
        y: u64,
    };
    let d = ((x.pow(2) + y.pow(2)) as f64).sqrt();
    if d == r as f64 {
        print!("1");
    } else if d != r as f64 && d < (2 * r) as f64 {
        println!("{}", 2);
    } else {
        println!("{}", (d / r as f64).ceil() as u64);
    }
}
