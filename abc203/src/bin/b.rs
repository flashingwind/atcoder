use proconio::input;

fn main() {
    input! {
        n: u32,
        k: u32,
    };
    let mut sum = 0;
    for f in 1..=n {
        for r in 1..=k {
            sum += f * 100 + r;
        }
    }
    println!("{}", sum);
}
