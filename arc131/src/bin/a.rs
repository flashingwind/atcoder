use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
    };
    println!("{}", 500_000_000 * b + a);
}
