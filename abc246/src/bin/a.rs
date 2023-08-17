use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        p: [(i64,i64);3]
    };
    let mut x = HashMap::new();
    let mut y = HashMap::new();
    for &(xx, yy) in p.iter() {
        *x.entry(xx).or_insert(0) += 1;
        *y.entry(yy).or_insert(0) += 1;
    }
    for (&xx, &cnt) in x.iter() {
        if cnt == 1 {
            print!("{} ", xx);
        }
    }
    for (&yy, &cnt) in y.iter() {
        if cnt == 1 {
            println!("{}", yy);
        }
    }
}
