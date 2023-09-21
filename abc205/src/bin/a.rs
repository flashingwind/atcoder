use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    };
    println!("{}", (a * b) as f64 / 100.0);
}
