use itertools::Itertools;
use proconio::input;
use rustc_hash::FxHashSet;
fn main() {
    input! {
        n: usize,
        s: String,
    };
    let mut set = FxHashSet::default();
    for pat in (0..n).combinations(n - 3) {
        let mut ss = String::new();
        for i in 0..n {
            if !pat.contains(&i) {
                ss += s.get(i..=i).unwrap_or("");
            }
        }
        // println!("{:?}: {ss}", pat);
        if ss.len() == 3 {
            set.insert(ss.clone());
        }
    }
    println!("{}", set.len());
}
