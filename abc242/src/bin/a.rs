use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        x: u32,
    };
    if x <= a {
        println!("1.000000000000");
    } else if x <= b {
        println!("{}", c as f64 / (b - a) as f64);
    } else {
        println!("0.000000000000");
    }
}
