use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        cards: [[u32;2];n],
    };
    let mut cnt = 2u64;
    for (i, c) in cards.iter().enumerate().skip(1) {
        if cards[i - 1][1] == cards[i][0] || cards[i - 1][0] == cards[i][1] {
            //cards[i - 1][0] == cards[i][0]|| cards[i - 1][1] == cards[i][1]
            // println!("{}=={}", i - 1, i);
            // } else if
            // cnt *= 2;
        } else {
            cnt *= 2;
        }
    }
    println!("{}", cnt);
}
