use proconio::input;

fn main() {
    input! {
        n: u64,
    };
    for t in 1..=2 {
        if (n * t) % 2 == 0 {
            println!("{}", t * n);
            return;
        }
    }
}
