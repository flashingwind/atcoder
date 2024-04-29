use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        a: Usize1,
    };
    println!("{}", (a + k - 1) % n + 1);
}
