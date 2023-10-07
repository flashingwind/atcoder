use proconio::{input, marker::Chars};
use rustc_hash::FxHashMap;

fn main() {
    input! {
        s: Chars,
    };
    let chokudai = vec!['c', 'h', 'o', 'k', 'u', 'd', 'a', 'i'];
    let mut set: FxHashMap<char, Vec<usize>> = FxHashMap::default();
    for &c in chokudai.iter() {
        set.entry(c).or_insert(
            s.iter()
                .enumerate()
                .filter(|v| *v.1 == c)
                .map(|(i, c)| i)
                .collect::<Vec<usize>>(),
        );
    }
    let mut cnt = vec![0; s.len()];
    for &i in set.entry('c').or_insert(vec![]).iter() {
        cnt[i] = 1;
    }

    for &c in chokudai.iter() {
        for &i in set.entry(c).or_insert(vec![]).iter() {}
    }
}
