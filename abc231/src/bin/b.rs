use itertools::Itertools;
use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        s: [String;n],
    };
    let mut ranking = BTreeMap::new();
    for ss in s.iter() {
        *ranking.entry(ss).or_insert(0) += 1;
    }
    let top = ranking
        .iter()
        .sorted_by(|a, b| a.1.cmp(&b.1))
        .rev()
        .collect::<Vec<_>>()[0]
        .0;
    println!("{}", top);
}
