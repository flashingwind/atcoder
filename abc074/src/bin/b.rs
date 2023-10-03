use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        xx: [usize;n],
    };
    let mut sum = 0;
    for y in 0..n {
        let x = xx[y];
        let d1 = x;
        let d2 = x.abs_diff(k);
        sum += d1.min(d2) * 2;
    }
    println!("{}", sum);
}
