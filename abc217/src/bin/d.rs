use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        l: usize,
        num_q: usize,
        q: [(u32,usize);num_q],
    };
    // let mut w = Vec::with_capacity(num_q);
    let mut w = BTreeSet::new();
    w.insert(l);
    w.insert(0);
    for (typ, x) in q.iter() {
        if *typ == 1 {
            w.insert(*x);
        } else if *typ == 2 {
            let lb = w.range(*x..).next().unwrap();
            let ub = w.range(..=*x).next_back().unwrap();
            println!("{}", lb - ub);
        }
    }
}
