use proconio::input;

fn main() {
    input! {
        x: u32,
    };
    if 0 <= x {
        println!("{}", x);
    } else {
        println!("0");
    }
}
