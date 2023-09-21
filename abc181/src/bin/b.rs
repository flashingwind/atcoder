use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(u64,u64);n],
    };
    let mut sum = 0;
    for (a, b) in ab.iter() {
        for i in *a..=*b {
            sum += i;
        }
    }
    println!("{}", sum);
}
