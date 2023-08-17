use proconio::input;

fn main() {
    input! {
        n: u32,
        sum: u32,
    };
    if n <= sum && sum <= 6 * n {
        println!("Yes");
    } else {
        println!("No");
    }
}
