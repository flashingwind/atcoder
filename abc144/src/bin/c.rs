use proconio::input;

fn main() {
    input! {
        n: u64,
    };
    let n_sqrt = (n as f64).sqrt().ceil() as u64;
    for i in (1..=n_sqrt).rev() {
        // println!("{}+{}-2=", n / i, i);
        if n % i == 0 {
            println!("{}", n / i + i - 2);
            break;
        }
    }
}
