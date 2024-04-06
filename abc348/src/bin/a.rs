use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    for t in 1..=n {
        print!("{}", if t % 3 == 0 { "x" } else { "o" });
    }
}
