use std::{collections::HashSet, iter::FromIterator};

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    };
    let i_set_all: HashSet<usize> = HashSet::from_iter(0..n.len());
    let mut max = 0;
    for len in 1..=n.len() - 1 {
        for pat in (0..n.len()).combinations(len) {
            let i_set_a: HashSet<_> = pat.iter().map(|v| *v).collect();
            let i_set_b = &i_set_all - &i_set_a;
            // if i_set_a.len() == 0 || i_set_b.len() == 0 {
            //     continue;
            // }

            let a: u64 = i_set_a
            .iter()
            .map(|i|n[*i])
            .sorted()
            .rev()
            .fold("".to_string(), |str, c| str+c.to_string().as_str())
            .parse()
            .unwrap();
            let b: u64 = i_set_b
            .iter()
            .map(|i|n[*i])
            .sorted()
            .rev()
            .fold("".to_string(), |str, c| str+c.to_string().as_str())
            .parse()
            .unwrap();
        // println!("set a={:?} set b={:?}", i_set_a
        //     .iter()
        //     .map(|i|n[*i])
        //     .sorted()
        //     .rev()
        //     .fold("".to_string(), |str, c| str+c.to_string().as_str()),
        // i_set_b
        //     .iter()
        //     .map(|i|n[*i])
        //     .sorted()
        //     .rev()
        //     .fold("".to_string(), |str, c| str+c.to_string().as_str()));
        // println!("{a}*{b}={}",a * b);
            max = max.max(a * b);
        }
    }
    println!("{max}");
}
