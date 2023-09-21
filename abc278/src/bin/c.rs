use proconio::input;
use rustc_hash::{FxHashMap, FxHashSet};

fn main() {
    input! {
        _n: usize,
        num_q: usize,
        qs: [(u32,u32,u32);num_q],
    };

    let mut is_following = FxHashMap::default();
    for (_t, q) in qs.iter().enumerate() {
        let a = q.1;
        let b = q.2;
        // println!("{_t}: typ{}: {a}->{b}: ", q.0);
        if q.0 == 1 {
            is_following
                .entry(a)
                .or_insert(FxHashSet::default())
                .insert(b);
        } else if q.0 == 2 {
            is_following
                .entry(a)
                .or_insert(FxHashSet::default())
                .remove(&b);
        } else if q.0 == 3 {
            if is_following
                .entry(a)
                .or_insert(FxHashSet::default())
                .contains(&b)
                && is_following
                    .entry(b)
                    .or_insert(FxHashSet::default())
                    .contains(&a)
            {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
