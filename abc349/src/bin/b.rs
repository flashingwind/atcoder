use itertools::Itertools;
use proconio::{input, marker::Chars};
use rustc_hash::FxHashMap;

fn main() {
    input! {
        s: Chars,
    };
    let mut map = FxHashMap::default();
    for &c in s.iter() {
        *map.entry(c).or_insert(0i64) += 1;
    }
    // println!("map={:?}", map);
    let mut cnt = map.values().map(|v| *v).collect_vec();
    // println!("cnt={:?}", cnt);
    cnt.sort();
    if cnt.len() % 2 != 0 {
        println!("No");
        return;
    }
    for c in cnt.chunks(2) {
        if c[0] != c[1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
