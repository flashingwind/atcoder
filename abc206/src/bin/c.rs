use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32;n],
    };
    a.sort();
    let mut cnt = a.len() * (a.len() - 1) / 2;
    a.insert(0, 0);
    let mut run_len = BTreeMap::new();
    for (i, _) in a.iter().enumerate().skip(1) {
        *run_len.entry(a[i]).or_insert(0) += 1;
    }
    for (_, q) in run_len.iter() {
        cnt -= *q * (*q - 1) / 2;
    }
    println!("{}", cnt);
}
