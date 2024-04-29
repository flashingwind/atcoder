use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        s: String,
    };
    let mut h = FxHashMap::default();
    for c in s.chars() {
        *h.entry(c).or_insert(0) += 1;
    }
    let mut single = 'X';
    for (c, cnt) in h.iter() {
        if *cnt == 1 {
            single = *c;
            break;
        }
    }
    for (i, c) in s.chars().enumerate() {
        if c == single {
            println!("{}", i + 1);
        }
    }
}
