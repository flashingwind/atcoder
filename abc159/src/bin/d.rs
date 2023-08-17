use itertools::Itertools;
use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        n: usize,
        mut a_vec: [u128;n],
    };
    let mut num_balls: FxHashMap<u128, u128> = FxHashMap::default();
    for a in a_vec.iter().sorted() {
        *num_balls.entry(*a).or_insert(0) += 1;
    }

    let mut pat_cnt: FxHashMap<u128, u128> = FxHashMap::default();

    let mut sum = 0;
    for (a, c) in num_balls.iter() {
        if 2 <= *c {
            let w = *c * (*c - 1) / 2;
            pat_cnt.entry(*a).or_insert(w);
            sum += w;
        }
    }
    // println!("pat_cnt={:?}", pat_cnt);
    for a in a_vec.iter() {
        let mut tmp_sum = sum;
        // println!("b={:?}", num_balls);
        if let Some(c) = num_balls.get(a) {
            tmp_sum -= *pat_cnt.entry(*a).or_insert(0);
            if 2 <= (*c - 1) {
                // println!("irregular[a={}]={}", *a, *c * (*c - 1) / 2);
                tmp_sum += (*c - 1) * (*c - 2) / 2;
            }
        }
        println!("{}", tmp_sum);
    }
}
