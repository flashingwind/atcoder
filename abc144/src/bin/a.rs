use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    };
    if a <= 9 && b <= 9 {
        println!("{}", a * b);
    } else {
        println!("-1");
    }
}
