use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s_arr: [String;n],
    };
    let mut cnt = BTreeMap::new();
    for t in s_arr.iter() {
        let cnt = cnt.entry(t).or_insert(0);
        if *cnt == 0 {
            println!("{}", t);
        } else {
            println!("{}({})", t, *cnt);
        }
        *cnt += 1;
    }
}
