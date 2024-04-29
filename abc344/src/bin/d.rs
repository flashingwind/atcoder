use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::{input, marker::Chars};
use rustc_hash::FxHashMap;

fn main() {
    input! {
        t: String,
        n: usize,
    };
    let mut s = vec![];
    let mut set = BTreeSet::new();
    for i in 0..n {
        input! {
            a: usize,
            mut ss: [String;a],
        };
        // ss.sort();
        // s.push(ss);
        set.insert(ss);
    }
    for ss in set.iter() {
        let v = t.match_indices(ss.iter().collect()).collect();
        if v.len() != 0 {
            set.remove(ss);
        }
    }
    let tt = t.clone();
    let mut idx = 0_usize;
    let mut tab = FxHashMap::default();
    for i in 0..n {
        //s
        for j in 0..s[i].len() {
            if tt.contains(&s[i][j]) {
                tab.entry()
            }
        }
    }
}
