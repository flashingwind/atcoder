use proconio::input;

fn main() {
    input! {
        x:i128,
    };

    let m = x % 10;
    if m == 0 {
        println!("{}", (x / 10));
    } else if 0 <= x {
        println!("{}", (x / 10) + 1);
    } else if x < 0 {
        println!("{}", (x / 10));
    }
}
