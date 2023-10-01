use proconio::input;

fn main() {
    input! {
        n: u64,
    };
    let mut balance = 100;
    for i in 1..n {
        balance += (balance * 1) / 100;
        if n <= balance {
            println!("{i}");
            return;
        }
    }
}
