use proconio::input;

fn main() {
    input! {
        n: usize,
        d: u32,
        t: [u32;n],
    };
    for i in 1..n {
        if t[i] - t[i - 1] <= d {
            println!("{}", t[i]);
            return;
        }
    }
    println!("-1");
}
