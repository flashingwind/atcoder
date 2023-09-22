use itertools::Itertools;
use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        n: usize,
        mut aa: [u64;n],
    }
    aa.sort();
    aa.reverse();
    let mut a_cnt = FxHashMap::default();
    for ai in aa.iter() {
        *a_cnt.entry(ai).or_insert(0) += 1;
    }

    let a_uniq: Vec<_> = aa.iter().unique().collect();
    let mut cnt: usize = 0;
    let mut ans = vec![0; n];
    for &&au in a_uniq.iter() {
        // println!("{au}: {cnt} += {}", 1);
        ans[cnt] += a_cnt[&au];
        cnt += 1;
    }
    for ans_k in ans.iter() {
        println!("{}", ans_k);
    }
}
