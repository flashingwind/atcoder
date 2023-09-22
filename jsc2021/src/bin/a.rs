use proconio::input;

fn main() {
    input! {
        x: u32,
        y: u32,
        z: u32,
    }
    if (y * z) % x == 0 {
        println!("{}", y * z / x - 1);
    } else {
        println!("{}", y * z / x);
    }
}
