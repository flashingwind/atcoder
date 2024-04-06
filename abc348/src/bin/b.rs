use num::pow::Pow;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ps: [(f64,f64);n],
    };
    for i in 0..n {
        let mut max = 0.0;
        let mut max_i = n + 1;
        for j in 0..n {
            if j == i {
                continue;
            }
            let d = (ps[i].0 - ps[j].0).pow(2) as f64 + (ps[i].1 - ps[j].1).pow(2) as f64;
            if max < d {
                max = d;
                max_i = j;
            }
            // println!("{}-{}: {d}", i + 1, j + 1);
            // println!("  dx:{} dy:{}", ps[i].0 - ps[j].0, ps[i].1 - ps[j].1);
        }
        println!("{}", max_i + 1);
    }
}
