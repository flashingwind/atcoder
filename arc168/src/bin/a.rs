use proconio::{input, marker::Chars};
use rustc_hash::FxHashMap;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        s: Chars
    };
    if n == 2 && s[0] == '<' {
        println!("0");
        return;
    } else if n == 2 && s[0] == '<' {
        println!("1");
        return;
    }
    let mut ns = vec![0; n];
    let mut cnt: BTreeMap<i32, i32> = BTreeMap::new();
    ns[n - 1] = 0;
    cnt.insert(ns[n - 1], 1);
    let mut min = 0;
    let mut ans = 0;
    let mut cache: std::collections::HashMap<i32, Option<i32>, _> = FxHashMap::default();
    for i in (0..n - 1).rev() {
        if s[i] == '>' {
            ns[i] = ns[i + 1] + 1;
        } else {
            min -= 1;
            ns[i] = min;
            //ns[i] < ns[i + 1]
        }
        let mut tmpcnt = 0;
        if s[i] == '<' {
            if let Some(Some(tmpcnt2)) = cache.get(&ns[i]) {
                tmpcnt = *tmpcnt2;
            } else {
                for (_, v) in cnt.range(..ns[i]) {
                    tmpcnt += *v;
                }
            }
        } else {
            for (_, v) in cnt.range(..ns[i]) {
                tmpcnt += *v;
            }
        }

        *cnt.entry(ns[i]).or_insert(0) += 1;
        *cache.entry(ns[i]).or_insert(None) = Some(tmpcnt);
        ans += tmpcnt;
        // print!("ns=    ");
        // for (j, &v) in ns.iter().enumerate() {
        //     if j == n - 1 {
        //         print!("{}", v);
        //     } else {
        //         print!("{} {} ", v, s[j]);
        //     }
        // }
        // println!();
        // println!("cnt=   {:?} ans={ans}\n", cnt);
    }
    println!("{}", ans);
}
