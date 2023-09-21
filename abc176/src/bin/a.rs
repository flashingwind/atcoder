use proconio::input;

fn main() {
    input! {
        n: u32,
        x: u32,
        t: u32,
    };
    println!("{}", t * (n / x + if n % x != 0 { 1 } else { 0 }));
}
