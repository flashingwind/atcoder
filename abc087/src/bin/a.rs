use proconio::input;

fn main() {
    input! {
        mut x: u32,
        a: u32,
        b: u32,
    };
    x -= a;
    x %= b;
    println!("{}", x);
}
