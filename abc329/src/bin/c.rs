use proconio::{input, marker::Chars};
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    if n == 1 {
        println!("1");
        return;
    }
    let mut pat = BTreeMap::default();
    let mut cnt = 0;
    for i in 0..n - 1 {
        cnt += 1;
        if s[i] != s[i + 1] {
            let tmp = pat.entry(s[i]).or_insert(0usize);
            *tmp = (*tmp).max(cnt);
            // println!("{i}:break cnt={cnt} {:?}", pat);
            cnt = 0;
            if i + 1 == n - 1 {
                let tmp = pat.entry(s[i + 1]).or_insert(0usize);
                *tmp = (*tmp).max(1);
                // println!("{i}+1: {} cnt={}", s[i + 1], 1);
                // println!("{i}+1: {:?}", pat);
            }
        } else {
            // println!("{i}: cnt={cnt}");
            if i + 1 == n - 1 {
                let tmp = pat.entry(s[i + 1]).or_insert(0usize);
                *tmp = (*tmp).max(cnt + 1);
                // println!("{i}+1: {} cnt={}", s[i + 1], cnt + 1);
                // println!("{i}+1: {:?}", pat);
            }
        }
    }
    println!("{}", pat.iter().map(|v| v.1).sum::<usize>());
}
