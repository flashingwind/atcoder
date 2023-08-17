use proconio::input;

fn main() {
    input! {
        x: u32,
    };
    println!("{}", 100 - x % 100);
}
