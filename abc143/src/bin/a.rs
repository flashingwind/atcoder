use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    };
    if b * 2 < a {
        println!("{}", a - b * 2);
    } else {
        println!("0");
    }
}
