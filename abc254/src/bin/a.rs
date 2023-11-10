use proconio::input;

fn main() {
    input! {
        n: u32,
    };
    println!("{:0>2}", n % 100);
}
