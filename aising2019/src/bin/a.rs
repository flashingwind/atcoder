use proconio::input;

fn main() {
    input! {
        n: usize,
        h: usize,
        w: usize,
    };
    println!("{}", (n - w + 1) * (n - h + 1));
}
