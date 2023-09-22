use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        c: [u32;n],
    }
    let mut set = FxHashMap::default();
    for i in 0..k {
        *set.entry(c[i]).or_insert(0usize) += 1;
    }
    // println!("{:?}", set);
    let mut max_len = set.len();
    for i in k..n {
        // print!("del{}..add{i} {:?}->", i - k, set);
        *set.entry(c[i]).or_insert(0) += 1;
        let tmp = set.entry(c[i - k]).or_insert(0);
        *tmp -= 1;
        if *tmp == 0 {
            set.remove(&c[i - k]);
        }
        let len = set.len();
        if max_len < len {
            max_len = len;
        }
    }
    println!("{}", max_len);
}
