use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let mut cnt = k;
    for _ in 1..n {
        cnt *= k - 1;
    }
    println!("{}", cnt);
}
