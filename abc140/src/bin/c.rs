use proconio::input;

fn main() {
    input! {
        n: usize,
        b: [u32;n-1],
    };
    let mut sum = 0;
    let mut before = b[0];
    for i in 0..n - 1 {
        if before < b[i] {
            sum += before;
        } else {
            sum += b[i];
        }
        before = b[i];
    }
    sum += before;
    println!("{}", sum);
}
