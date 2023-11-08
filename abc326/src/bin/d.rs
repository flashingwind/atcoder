use std::collections::HashSet;

use itertools::Itertools;
use proconio::{input, marker::Chars};
use rustc_hash::FxHashMap;

fn main() {
    input! {
        n: usize,
        r: CHars,
        c: Chars
    };
    let mut map = vec![vec!['.'; n]; n];
    let mut abc = Vec::from(&['A', 'B', 'C']);
    abc.extend(vec!['.'; n - 3]);
    for i in 0..n {
        for pat in (0..n).combinations(n) {
            for &j in pat.iter() {
                map[i][j] = abc[j];
                if abc[j] != c[i] {
                    break;
                }
            }
        }
    }
    if is_ok(map) {
        println!("Yes");
        return;
    }
}

fn is_ok(map: Vec<Vec<char>>) -> bool {
    let mut r_cnt = FxHashMap::default();
    for cc in r.iter() {
        if cc != '.' {
            *r_cnt.entry(cc).or_insert(0) += 1;
        }
    }
    if *r_cnt.entry(&'A') != 1 || *r_cnt.entry(&'B') != 1 || *r_cnt.entry(&'C') != 1 {
        println!("No");
        return false;
    }
    let mut c_cnt = FxHashMap::default();
    for cc in c.iter() {
        if cc != '.' {
            *c_cnt.entry(cc).or_insert(0) += 1;
        }
    }
    if *c_cnt.entry(&'A') != 1 || *c_cnt.entry(&'B') != 1 || *c_cnt.entry(&'C') != 1 {
        println!("No");
        return false;
    }
    return true;
}
