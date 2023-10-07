use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    };
    if a * b == 15 {
        println!("*");
    } else if a + b == 15 {
        println!("+");
    } else {
        println!("x");
    }
}
