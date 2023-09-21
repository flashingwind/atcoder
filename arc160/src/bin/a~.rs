use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        n: usize,
        ranking: usize,
        a: [u64;n],
    };
    let mut rank = FxHashMap::default();

    for r in 0..n {
        for k in 0..=r {
            *rank.entry((0, r)).or_insert(0) += (k as u64 + 1) * a[r - k];
        }
        for k in r + 1..n {
            *rank.entry((0, r)).or_insert(0) += (k as u64 + 1) * a[k];
        }
    }
    for l in 1..n {
        for r in l..n {
            *rank.entry((l, r)).or_insert(0) += *rank.entry((l - 1, r)).or_insert(0);
            for k in l..=r {
                *rank.entry((l, r)).or_insert(0) += (k as u64 + 1) * a[r - k];
            }
            for k in r + 1..n {
                *rank.entry((l, r)).or_insert(0) += (k as u64 + 1) * a[k];
            }
        }
    }
    let mut v: Vec<(&(usize, usize), &u64)> = rank.iter().collect();
    v.sort_by(|a, b| a.1.cmp(b.1));
    v.reverse();
    // println!("{:?}", v);
    // println!("{:?}", v[ranking - 1]);
    let b_lr = *v[ranking - 1].0;
    let mut is_first = true;
    for k in 0..b_lr.0 {
        if is_first {
            is_first = false;
            print!("{}", a[k]);
        } else {
            print!(" {}", a[k]);
        }
    }
    for k in (b_lr.0..=b_lr.1).rev() {
        if is_first {
            is_first = false;
            print!("{}", a[k]);
        } else {
            print!(" {}", a[k]);
        }
    }
    for k in b_lr.1 + 1..n {
        if is_first {
            is_first = false;
            print!("{}", a[k]);
        } else {
            print!(" {}", a[k]);
        }
    }
    println!();
}
