use proconio::input;

fn main() {
    input! {
        n: u32,
    };
    println!("{}", if n <= 999 { "ABC" } else { "ABD" });
}
