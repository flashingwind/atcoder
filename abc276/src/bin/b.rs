use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        pair: [(usize,usize);m],
    };
    let mut es = HashMap::with_capacity(n);
    for &(a, b) in pair.iter() {
        es.entry(a).or_insert(Vec::new()).push(b);
        es.entry(b).or_insert(Vec::new()).push(a);
    }
    for i in 1..=n {
        let v = es.entry(i).or_insert(Vec::new());
        print!("{}", v.len());
        v.sort();
        for a in v.iter() {
            print!(" {}", a);
        }
        println!()
    }
}
