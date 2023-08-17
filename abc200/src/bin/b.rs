use proconio::input;

fn main() {
    input! {
        mut n: u128,
        k: u32,
    };
    for _ in 0..k {
        if n % 200 == 0 {
            n /= 200;
        } else {
            n = n * 1000 + 200;
        }
    }
    println!("{}", n);
}
