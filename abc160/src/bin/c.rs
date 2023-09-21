use proconio::input;

fn main() {
    input! {
        k: u32,
        n: usize,
        mut a: [u32;n],
    };
    a.sort();
    let mut min_len = k;
    for i in 0..n - 1 {
        let len = k - (a[i + 1] - a[i]);
        min_len = min_len.min(len);
        // println!("{}-{}: {}", i, i + 1, len);
    }
    let len = k - ((k - a[n - 1]) + a[0]);
    // println!("{}-{}: {}", n - 1, 0, len);
    min_len = min_len.min(len);
    println!("{}", min_len);
}
