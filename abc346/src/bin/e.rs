use std::collections::BTreeMap;

use proconio::{input, marker::Usize1};
use rustc_hash::FxHashMap;

fn main() {
    input! {
        h: usize,
        w: usize,
        m: usize,
        tax: [(u32,Usize1,usize);m],
    };
    let mut porc_r = vec![usize::MAX; w];
    let mut porc_c = vec![usize::MAX; w];
    let mut drawed_row = 0;
    let mut drawed_col = 0;
    let mut cnt = BTreeMap::new();
    let tax2 = vec![];
    for i in tax.iter().rev() {
        tax2.push();
    }
    for &(t, a, x) in tax.iter() {
        if t == 1 {
            //行
            porc_r[a] = x;
            *cnt.entry(x).or_insert(0) += h - drawed_col;
            drawed_raw += 1;
        } else {
            //列
            porc_c[a] = x;
            *cnt.entry(x).or_insert(0) += h - drawed_row;
            drawed_col += 1;
        }
    }
    for i in 0..m {}
}
