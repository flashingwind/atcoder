use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        num_w: usize,
        sum_w: u32,
        mut w: [u32;num_w],
    };
    let mut good = BTreeSet::new();
    w.push(0);
    w.push(0);
    let mut cnt = 0;
    for (i1, w1) in w.iter().enumerate() {
        if *w1 == 0 {
            continue;
        }
        for (i2, w2) in w.iter().enumerate() {
            for (i3, w3) in w.iter().enumerate() {
                let sum = *w1 + *w2 + *w3;
                // print!("{}+{}+{}={}", *w1, *w2, *w3, sum);
                if i1 != i2 && i2 != i3 && i1 != i3 && sum <= sum_w && good.insert(sum) {
                    cnt += 1;
                    // print!("cnt");
                }
                // println!();
            }
        }
    }
    println!("{}", cnt);
}
