use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        k: usize,
    };

    let mut cache = vec![Vec::new(); 0];
    for i in 1..=9 {
        cache.push(vec![i]);
        // print!("{}", cache.last().unwrap().iter().join(""));
        if cache.len() == k {
            println!("{i}");
            return;
        }
    }
    loop {
        for d2 in 1..=9 {
            for i in 0..cache.len() {
                let d = *cache[i].last().unwrap();
                if d2 < d {
                    // println!("  +{:?}={d}+{d2}", str);
                    let mut v = cache[i].to_owned();
                    v.push(d2);
                    cache.push(v);
                }
                if cache.len() == k + 1 {
                    // println!("cache.len()={} {:?}", cache.len() - 1, cache);
                    println!("{}", cache.last().unwrap().iter().join(""));
                    return;
                }
            }
        }
    }
}
