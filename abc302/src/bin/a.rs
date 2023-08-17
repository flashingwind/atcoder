use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
    };
    println!("{}", a / b + if a % b != 0 { 1 } else { 0 });
}
