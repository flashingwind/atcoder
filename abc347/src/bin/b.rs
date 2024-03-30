use itertools::Itertools;
use proconio::{input, marker::Chars};
use rustc_hash::FxHashSet;

fn main() {
    input! {
        s: Chars,
    };
    let mut set = FxHashSet::default();

    for pat in (0..s.len()).combinations_with_replacement(2) {
        if pat[0] <= pat[1] {
            let mut str = String::new();
            for i in pat[0]..=pat[1] {
                str += s[i].to_string().as_str();
            }
            // println!("{}-{}:{}", pat[0], pat[1], str);
            set.insert(str);
        }
    }
    println!("{}", set.len());
}
