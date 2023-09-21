use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        num_a: usize,
        num_q: usize,
        mut arr: [usize;num_a],
        qs: [(usize,usize);num_q],
    };
    arr.insert(0, 0);

    let mut cache: FxHashMap<usize, Vec<usize>> = FxHashMap::default(); // vec![Vec::new(); 10usize.pow(9)];
    for (i, a1) in arr.iter().enumerate().skip(1) {
        cache.entry(*a1).or_default().push(i);
    }
    // println!("{:?}", cache);
    for (x, k) in qs {
        // println!("x={x} k={k}:");
        if let Some(&i) = cache.entry(x).or_default().get(k - 1) {
            println!("{}", i);
        } else {
            println!("-1");
        }
    }
}
