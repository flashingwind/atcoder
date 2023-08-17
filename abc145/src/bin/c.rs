use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i32,i32);n],
    };
    let mut dists = vec![vec![None; n]; n];
    let mut cnt = 0u64;
    let mut avg = 0.0;
    for pat in (0..n).permutations(n) {
        let mut past_i = n;
        let mut sum = 0.0;
        for i in pat {
            if past_i == n {
                past_i = i;
                continue;
            }
            if let Some(d) = dists[past_i][i] {
                sum += d;
                // println!("{past_i}->{i}: {d}");
            } else {
                let d = ((xy[past_i].0 - xy[i].0).pow(2) as f64
                    + (xy[past_i].1 - xy[i].1).pow(2) as f64)
                    .sqrt();
                dists[past_i][i] = Some(d);
                sum += d;
                // println!("*{past_i}->{i}: {d}");
            }
            past_i = i;
        }
        cnt += 1;
        avg += sum;
        // println!("sum={}", sum);
    }
    avg /= cnt as f64;
    println!("{}", avg);
}
