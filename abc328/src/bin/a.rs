use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u32,
        s: [u32;n],
    };
    let mut sum = 0;
    for i in 0..n {
        if s[i] <= x {
            sum += s[i];
        }
    }
    println!("{}", sum);
}
