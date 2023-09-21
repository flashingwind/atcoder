use proconio::input;

fn main() {
    input! {
        n: u64,
    };
    let diff = 2025 - n;
    // println!("diff={diff}");
    for i in 1..=9 {
        for j in 1..=9 {
            if i * j == diff {
                println!("{} x {}", i, j);
            }
        }
    }
}
