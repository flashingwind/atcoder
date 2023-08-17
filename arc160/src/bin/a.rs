use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        n: usize,
        ranking: usize,
        a: [u32;n],
    };
    let mut rank = FxHashMap::default();

    for r in 0..n {
        for k in 0..=r {
            rank.entry((0, r))
                .or_insert(String::new())
                .push_str(a[r - k].to_string().as_str());
        }
        for k in r + 1..n {
            rank.entry((0, r))
                .or_insert(String::new())
                .push_str(a[k].to_string().as_str());
        }
    }
    let mut str;
    for l in 1..n {
        for r in l..n {
            str = rank
                .entry((l - 1, r))
                .or_insert(String::new())
                .as_str()
                .to_owned();
            rank.entry((l, r))
                .or_insert(String::new())
                .push_str(&str.to_owned());
            for k in l..=r {
                rank.entry((l, r))
                    .or_insert(String::new())
                    .push_str(a[r - k].to_string().as_str());
            }
            for k in r + 1..n {
                rank.entry((l, r))
                    .or_insert(String::new())
                    .push_str(a[k].to_string().as_str());
            }
        }
    }
    let mut v: Vec<(&(usize, usize), &String)> = rank.iter().collect();
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
