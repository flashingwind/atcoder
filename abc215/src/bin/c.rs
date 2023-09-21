use itertools::Itertools;
use proconio::{input, marker::Chars};
use rustc_hash::FxHashSet;

fn main() {
    input! {
        mut cs: Chars,
        k: usize,
    };

    cs.sort();
    let s = cs.iter().collect::<String>();

    let mut cnt = 0;
    let mut cache: FxHashSet<String> = FxHashSet::default();
    cache.reserve(k);
    // let mut cache: BTreeSet<String> = BTreeSet::new();

    for p in (0..s.len()).permutations(s.len()) {
        let ss: String = p.iter().map(|&i| s.get(i..=i).unwrap()).collect();
        if !cache.contains(&ss) {
            cnt += 1;
            // println!("{cnt}:{} {:?}", ss, cache);
            if cnt == k {
                println!("{}", ss);
                break;
            }
            cache.insert(ss);
        }
    }
}
