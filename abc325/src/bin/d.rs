use itertools::Itertools;
use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        n: usize,
        mut td: [(usize,usize);n],
    };
    let mut cnt = 0;
    let mut t = 1;
    td.sort();
    let mut ca = FxHashMap::default();
    let mut start = 0;
    let mut is_changed = false;
    loop {
        is_changed = false;
        for i in start..n {
            if td[i].0 <= t {
                if t < td[i].1 {
                    ca.insert(i, td[i].1);
                } else {
                    break;
                }
            } else {
                ca.remove(&i);
                start = i;
            }
        }
        if let Some(k) = ca
            .iter()
            .sorted_by(|a, b| a.1.cmp(&b.1))
            .map(|v| *v.0)
            .take(1)
            .collect::<Vec<_>>()
            .first()
        {
            cnt += 1;
            is_changed = true;
        }
        t += 1;
        if !is_changed {
            println!("{}", cnt);
            return;
        }
    }
}
