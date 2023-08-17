use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        strs: [String;n],
    };
    let mut cnt: BTreeMap<String, u32> = BTreeMap::new();
    for s in strs {
        *cnt.entry(s).or_insert(0) += 1;
    }
    let rank: Vec<(String, u32)> = cnt.into_iter().sorted_by(|a, b| a.1.cmp(&b.1)).collect();
    let mut max = 0;
    for (_, c) in rank.iter() {
        if max < *c {
            max = *c;
        }
    }
    for (s, _) in rank.iter().filter(|(_, c)| *c == max).sorted() {
        println!("{}", s);
    }
}
