use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    if n == 1 {
        println!("Hello World");
    } else {
        input! {
            a: u32,
            b: u32,
        };
        println!("{}", a + b);
    }
}
