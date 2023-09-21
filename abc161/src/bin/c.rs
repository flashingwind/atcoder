use proconio::input;

fn main() {
    input! {
        mut n: i64,
        k: i64,
    };
    let mut min_n = std::i64::MAX;
    n = n % k;
    for _ in 1..=2 {
        if n <= k {
            n = k - n;
        } else {
            n = n - k;
        }
        if n < min_n {
            min_n = n;
        }
        // println!("{}", n);
    }
    println!("{}", min_n);
}
