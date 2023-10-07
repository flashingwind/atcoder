use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1;n],
    };
    let mut is_ok = vec![false; n];
    let mut is_checked_f = vec![false; 100001];
    for i in (0..n).rev() {
        if is_checked_f[a[i]] {
            is_ok[i] = true;
        }
        is_checked_f[a[i]] = true;
    }

    println!("{}", is_ok.iter().filter(|v| **v).count());
}
