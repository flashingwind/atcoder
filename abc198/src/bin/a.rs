use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut pat = 0;
    for _ in 1..n {
        pat += 1;
    }
    println!("{}", pat);
}
