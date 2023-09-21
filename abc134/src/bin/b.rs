use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
    };
    let dd = 2 * d + 1;
    if n % dd == 0 {
        println!("{}", n / dd);
    } else {
        println!("{}", n / dd + 1);
    }
}
