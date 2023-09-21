use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [u32;n],
    };
    let mut sum = 0;
    for i in 0..n {
        for j in 0..i {
            sum += d[i] * d[j];
        }
    }
    println!("{}", sum);
}
