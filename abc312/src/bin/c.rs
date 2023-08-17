use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [u32;n],
        mut b: [u32;m],
    };
    a.sort();
    b.sort();
    let mut a_cnt = BTreeMap::new();
    let mut b_cnt = BTreeMap::new();
    {
        //URI
        let mut a_sum = 0i64;
        // a_cnt.entry(a.first().unwrap() - 1).or_insert(0);
        for &ai in a.iter() {
            a_sum += 1;
            *a_cnt.entry(ai).or_insert(0) = a_sum;
            a_cnt.entry(ai - 1).or_insert(a_sum - 1);
        }
        //KAI
        let mut b_sum = 0i64;
        // b_cnt.entry(b.last().unwrap() + 1).or_insert(0);
        for &bi in b.iter().rev() {
            b_sum -= 1;
            *b_cnt.entry(bi).or_insert(0) = b_sum;
            b_cnt.entry(bi + 1).or_insert(b_sum + 1);
        }
    }
    let mut mergrd = a_cnt.clone();
    mergrd.extend(b_cnt.iter());
    let mut last_a_cnt = 0;
    let mut last_b_cnt = *b_cnt.entry(*b_cnt.keys().next().unwrap()).or_insert(0);
    for (x, _) in mergrd.iter() {
        if let Some(&ca) = a_cnt.get(x) {
            last_a_cnt = ca;
            // println!("{x}: last_a_cnt={last_a_cnt}");
        }
        if let Some(&cb) = b_cnt.get(x) {
            last_b_cnt = cb;
            // println!("{x}: last_b_cnt={last_b_cnt}");
        }
        let sum = last_a_cnt + last_b_cnt;
        // println!("x={x} {last_a_cnt}:{last_b_cnt} sum={sum}");
        if 0 <= sum {
            println!("{}", x);
            return;
        }
        // println!();
    }
}
