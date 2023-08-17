//use std::{convert::TryInto, u128};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut hap: [u128;n],
    };
    let mut orderd_pair: Vec<((usize, usize), u128)> = Vec::new();
    hap.sort();
    // let mut m_sqrt = m;
    // unsafe {
    //     m_sqrt = TryInto::<usize>::try_into((m as f64).sqrt().floor().to_int_unchecked::<usize>())
    //         .unwrap()
    //         + 1;
    //     println!("m_sqrt={m_sqrt}");
    // }
    let itr = hap.iter().rev().take(m).enumerate();

    for (p1, h1) in itr.to_owned() {
        for (p2, h2) in itr.to_owned() {
            orderd_pair.push(((p1, p2), h1 + h2));
            println!("push {} {}", p1, p2);
        }
    }

    orderd_pair.sort_by(|a, b| a.1.cmp(&b.1));
    // println!("{:?}", orderd_pair);

    let mut sum = 0;

    for ((_p1, _p2), h) in orderd_pair.iter().rev().take(m) {
        sum += h;
        // println!("{},{} h={}+{}={h}", *p1, *p2, hap[*p1], hap[*p2]);
    }
    println!("{}", sum);
}
