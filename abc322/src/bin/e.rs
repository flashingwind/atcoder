use itertools::Itertools;
use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: u32,
        mut plan: [[u32;k+1];n],
    };
    plan.sort();
    let mut min = u32::MAX;
    let mut stats = vec![FxHashMap::default(); n];
    for t in 1..=n.max(20) {
        for pat in (0..n).combinations(t) {
            let mut c = 0;
            // let mut s = stat.entry(&pat).or_insert(vec![0; k]);
            let mut s = vec![0; k];
            for &i in pat.iter() {
                for j in 0..k {
                    s[j] += plan[i][j + 1];
                }
                s[j].min(5);
                c += plan[i][0];
            }
            let mut is_ok = true;
            for j in 0..k {
                if s[j] < p {
                    is_ok = false;
                    break;
                }
            }
            // println!("{:?}:{:?}:ok={is_ok} c={c}", pat, s);
            if is_ok && c < min {
                min = c;
            }
        }
        // if min != u32::MAX {
        //     println!("{}", min);
        //     return;
        // }
    }
    if min == u32::MAX {
        println!("-1");
    } else {
        println!("{}", min);
        return;
    }
}
