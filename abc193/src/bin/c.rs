use std::default;

use num::integer::Roots;
use proconio::input;
use rustc_hash::FxHashSet;

fn main() {
    input! {
        n: u64,
    };
    let mut cnt = 0;
    let mut is_checked = FxHashSet::default();
    for a in 2..=100000.min(n.sqrt() + 1) {
        for b in 2..=34.min(n) {
            let ab = a.pow(b as u32);
            if n < ab {
                break;
            }
            if !is_checked.contains(&ab) {
                is_checked.insert(ab);
                cnt += 1;
            }
        }
    }
    println!("{}", n - cnt);
}
