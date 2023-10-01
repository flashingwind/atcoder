use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [i32;n],
        c: [i32;n],
    };
    let mut max_diff = i32::MIN;
    for t in 0..=n {
        for pat in (0..n).combinations(t) {
            let mut sum_c = 0;
            let mut sum_v = 0;
            for &i in pat.iter() {
                sum_c += c[i];
                sum_v += v[i];
            }
            if max_diff < (sum_v - sum_c) {
                max_diff = sum_v - sum_c;
            }
        }
    }
    if max_diff == i32::MIN {
        println!("0");
    } else {
        println!("{max_diff}");
    }
}
