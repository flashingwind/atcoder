use proconio::input;

fn main() {
    input! {
        a: i32,
        b: u32,
    };
    println!("{}", a.pow(b));
}
