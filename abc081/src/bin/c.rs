use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u32;n],
    };
    let mut hist = BTreeMap::default();
    for i in 0..n {
        *hist.entry(&a[i]).or_insert(0) += 1;
    }
    // println!("{:?}", hist);
    let mut len = hist.len();
    let mut sum = 0usize;
    let mut itr = hist.iter().sorted_by(|a, b| b.1.cmp(&a.1));
    // for len in 0..k {
    //     if let Some((key, v)) = itr.next() {}
    // }
    for (k, v) in itr.skip(k) {
        sum += *v;
    }
    println!("{sum}");
}
