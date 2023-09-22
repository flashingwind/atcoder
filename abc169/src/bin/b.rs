use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u64;n],
    }
    a.sort();
    let mut p = 1u64;
    const MAX: u64 = 1000000000000000000;
    for i in 0..n {
        if a[i] == 0 {
            println!("0");
            return;
        }
        if MAX / a[i] < p {
            println!("-1");
            return;
        }
        p *= a[i];
    }
    println!("{}", p);
}
