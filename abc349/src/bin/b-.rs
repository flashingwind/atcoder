use itertools::Itertools;
use proconio::{input, marker::Chars};
use rustc_hash::FxHashMap;

fn main() {
    input! {
        s: Chars,
    };
    let mut map = FxHashMap::default();
    for &c in s.iter() {
        *map.entry(c).or_insert(0) += 1;
    }
    println!("map={:?}", map);
    let mut cnt = map.values().map(|v| *v).collect_vec();
    println!("cnt={:?}", cnt);
    cnt.sort();
    let mut l = 0;
    while l < cnt.len() {
        let mut runcnt = 0;
        println!("cnt[{l}]={}:", cnt[l]);
        let mut r_last = l;
        for r in l..cnt.len() {
            println!(" cnt[{r}]={}: ", cnt[r]);
            if cnt[r] != cnt[l] {
                r_last = r;
                break;
            } else {
                runcnt += 1;
                r_last = r;
            }
        }
        println!("runcnt={runcnt}");
        if runcnt != 0 && runcnt != 2 {
            println!("No");
            return;
        }
        l = r_last + 1;
    }
    println!("Yes");
}
