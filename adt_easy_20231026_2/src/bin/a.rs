use proconio::input;

fn main() {
    input! {
        n: u32,
    };
    if n % 5 <= 2 {
        println!("{}", (n / 5) * 5);
    } else {
        println!("{}", (n / 5 + 1) * 5);
    }
}
