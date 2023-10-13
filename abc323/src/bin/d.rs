use proconio::input;
use std::collections::{BTreeMap, VecDeque};

fn main() {
    input! {
        n: usize,
        size_cnt: [(u64,u64);n],
    };
    let mut slimes: BTreeMap<u64, u64> = size_cnt.iter().map(|&(s, c)| (s, c)).collect();

    let mut to_proccess: VecDeque<u64> = size_cnt.iter().map(|&(s, c)| s).collect();
    while let Some(size) = to_proccess.pop_front() {
        // println!("{k}: {:?}", slimes);
        let cnt = slimes.entry(size).or_insert(0);
        if *cnt / 2 != 0 {
            *slimes.entry(size * 2).or_insert(0) += *cnt / 2;
            *slimes.entry(size).or_insert(0) %= 2;
            to_proccess.push_back(size * 2);
        }
        // println!("{k}: {:?}", slimes);
    }
    println!("{}", slimes.iter().map(|(_, c)| c).sum::<u64>());
}
