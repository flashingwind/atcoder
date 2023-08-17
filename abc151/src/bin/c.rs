use std::vec;

use proconio::input;

fn main() {
    input! {
        num_probs: usize,
        num_logs: usize,
        logs: [(usize,String);num_logs],
    };
    let mut probs = vec![(0u32, 0u32); num_probs]; //(AC,WA)
    for l in logs.iter() {
        let (p, res) = l.to_owned();
        if res == "WA" && probs[p - 1].0 == 0 {
            // WA, before first AC
            probs[p - 1].1 += 1;
        } else if res == "AC" {
            // AC
            probs[p - 1].0 = 1;
        }
    }
    let mut cnt_ac = 0;
    let mut cnt_wa = 0;
    for (ac, wa) in probs.iter() {
        if *ac == 1 {
            cnt_ac += *ac;
            cnt_wa += *wa;
        }
    }
    println!("{} {}", cnt_ac, cnt_wa);
}
