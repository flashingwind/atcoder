use proconio::input;

fn main() {
    input! {
        n: u64,
    };
    // let n_sqrt = (n as f64).sqrt().ceil() as u64;
    for i in 1..=9 {
        if n % i == 0 && n / i <= 9 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
