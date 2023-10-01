use proconio::input;

fn main() {
    input! {
        n: u32,
        k: u32
    };
    if n % k == 0 {
        println!("{}", 0);
    } else {
        println!("{}", 1);
    }
}
