use maplit::hashmap;

use proconio::{input, marker::Chars};
use rustc_hash::FxHashMap;

fn main() {
    input! {
        mut s: Chars,
        mut t: Chars,
    };
    s.sort();
    t.sort();
    // println!("s={:?}", s);
    // println!("t={:?}", t);
    // let atcoder = ['a'tcoder".to_string();
    let atcoder = ['a', 't', 'c', 'o', 'd', 'e', 'r'];
    let cnt_at_s = s.iter().filter(|v| **v == '@').count();
    let cnt_at_t = t.iter().filter(|v| **v == '@').count();
    // println!("cnt_at_s={cnt_at_s} cnt_at_t={cnt_at_t}");
    s.drain(0..cnt_at_s);
    t.drain(0..cnt_at_t);
    let mut all = FxHashMap::default();
    for ss in s.iter() {
        *all.entry(ss)
            .or_insert(hashmap! {'S'=> 0})
            .entry('S')
            .or_insert(0) += 1;
    }
    for tt in t.iter() {
        *all.entry(tt)
            .or_insert(hashmap! {'T'=> 0})
            .entry('T')
            .or_insert(0) += 1;
    }
    // println!("all={:?}\nall.len={}", all, all.len());

    let mut cnt_only_in_s = 0;
    let mut cnt_only_in_t = 0;
    for (c, v) in all.iter_mut() {
        let cnt_t = *v.entry('T').or_insert(0);
        let cnt_s = *v.entry('S').or_insert(0);
        if cnt_s < cnt_t {
            if !atcoder.contains(&**c) {
                // println!("{c}:{:?} not in atcoder", v);
                println!("No");
                return;
            } else {
                // println!("{c}:{:?} in atcoder", v);
                cnt_only_in_t += cnt_t - cnt_s;
            }
        } else if cnt_t < cnt_s {
            if !atcoder.contains(&**c) {
                // println!("{c}:{:?} not in atcoder", v);
                println!("No");
                return;
            } else {
                // println!("{c}:{:?} in atcoder", v);
                cnt_only_in_s += cnt_s - cnt_t;
            }
        }
    }
    // println!("cnt_only_in_s={cnt_only_in_s} cnt_only_in_t={cnt_only_in_t}");
    if cnt_at_t >= cnt_only_in_s && cnt_at_s >= cnt_only_in_t {
        println!("Yes");
    } else {
        println!("No");
    }
}
