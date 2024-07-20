use itertools::Itertools;
use proconio::{input, marker::Chars};
use rustc_hash::FxHashMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    };
    let mut sum = factorial(n);
    // let mut cache = FxHashMap::default();
    for pat in (0..n).permutations(n - k) {
        let mut ss = s.clone();
        for i in pat.iter().sorted().rev() {
            ss.remove(*i);
        }
        ss.sort();
        // if let Some((str, v)) = cache.get(&ss.clone()) {
        //     continue;
        // }
        let mut map = FxHashMap::default();
        for c in ss.iter() {
            *map.entry(c).or_insert(0) += 1;
        }
        let mut cnt_odd = 0;
        for (_, v) in map.iter() {
            if v % 2 != 0 {
                cnt_odd += 1;
            }
        }
        if cnt_odd <= 1 {
            //is kaibun
            sum -= factorial(ss.len()) * (n - k + 1);
            // println!("{:?}", ss);
            // cache.insert(pat.clone(), -1 * (n - k) as i64);
        }
    }
    println!("{}", sum);
}

fn factorial(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}
