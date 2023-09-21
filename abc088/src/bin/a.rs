use proconio::input;

fn main() {
    input! {
        n: u32,
        a: u32,
    };
    if n % 500 <= a {
        println!("Yes");
    } else {
        println!("No");
    }
}
