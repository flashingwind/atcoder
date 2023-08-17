use proconio::input;

fn main() {
    input! {
        x: u32,
    };
    if 90 <= x {
        println!("expert");
    } else if 70 <= x {
        println!("{}", 90 - x);
    } else if 40 <= x {
        println!("{}", 70 - x);
    } else {
        println!("{}", 40 - x);
    }
}
