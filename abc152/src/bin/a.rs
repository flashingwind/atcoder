use proconio::input;

fn main() {
    input! {
        n: u32,
        m: u32,
    };
    if n == m {
        println!("Yes");
    } else {
        println!("No");
    }
}
