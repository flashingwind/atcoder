use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize;n],
    };
    a.sort();
    let mut cnt = BTreeMap::new();
    for i in 1..n {
        let old = *cnt.entry(a[i - 1]).or_insert(1);
        cnt.insert(a[i], old + 1);
    }

    let mut max = 0;
    for i in 0..n {
        let left;
        if i == 0 {
            left = 0;
        } else {
            left = *cnt.entry(a[i]).or_insert(0);
        }
        let mut right = left;
        for j in i..n {
            if a[j] < a[i] + m {
                right = *cnt.entry(a[j]).or_insert(0);
            } else {
                break;
            }
            // println!("{}:{left}~{}:{right}:{:?} ={}", a[i], a[j], (right - left));
        }
        max = max.max(right - left);
    }
    println!("{max}");
}
