use proconio::input;

fn main() {
    input! {
        n: u32,
    };
    if 42 <= n {
        println!("AGC{:03}", n + 1);
    } else {
        println!("AGC{:03}", n);
    }
}
