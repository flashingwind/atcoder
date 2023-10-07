use proconio::{input, marker::Usize1};
use rustc_hash::FxHashMap;

fn main() {
    input! {
        n: usize,
        mut a: [Usize1;n],
        mut b: [Usize1;n],
        mut c: [Usize1;n],
    };
    a.sort();
    b.sort();
    c.sort();
    let mut cache_a: FxHashMap<usize, usize> = FxHashMap::default();
    let mut cache_b: FxHashMap<usize, usize> = FxHashMap::default();
    for i in 1..a.len() {
        if i != 0 {
            *cache_a.entry(a[i]).or_insert(0) = *cache_a.entry(a[i - 1]).or_insert(0);
        }
        for j in 1..b.len() {
            let bj_cache = cache_b.entry(b[j]).or_insert(0);
            if j != 0 {
                let bj_cache_before = *cache_b.entry(b[j]).or_insert(0);
                *bj_cache += bj_cache_before;
            }
            if a[i] < b[j] {
                for k in (0..c.len()).rev() {
                    if b[j] < c[k] {
                        *bj_cache += 1;
                    }
                }
                *cache_a.entry(a[i]).or_insert(0) += *cache_b.entry(b[j]).or_insert(0);
            }
        }
    }
    println!("{}", cache_a.values().sum::<usize>());
}
