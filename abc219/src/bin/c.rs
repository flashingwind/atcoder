use std::cmp::Ordering;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        ord: Chars,
        n: usize,
        mut s: [Chars;n],
    };

    s.sort_by(|a, b| {
        let mut itra = a.iter();
        let mut itrb = b.iter();
        loop {
            if let Some(c_a) = itra.next() {
                if let Some(c_b) = itrb.next() {
                    let i_a = ord.clone().into_iter().position(|x| x == *c_a);
                    let i_b = ord.clone().into_iter().position(|x| x == *c_b);
                    if i_a == i_b {
                        continue;
                    } else {
                        // println!("{c_a}:{:?}--{c_b}:{:?}: {:?}", i_a, i_b, i_a.cmp(&i_b));
                        return i_a.cmp(&i_b);
                    }
                } else {
                    //|a|>|b|
                    return Ordering::Greater;
                }
            } else {
                //|a|<|b|
                return Ordering::Less;
            }
        }
    });
    for name in s {
        println!("{}", name.iter().collect::<String>());
    }
}
