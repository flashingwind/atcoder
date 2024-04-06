use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        n: usize,
        ac: [(u32,usize);n],
    };
    // let mut min = vec![u32::MAX; 10usize.pow(9)];
    // let mut col = vec![];
    let mut min = FxHashMap::default();
    for i in 0..n {
        // println!("i={i}");
        // if min[ac[i].1] == u32::MAX {
        // if  == u32::MAX {
        //     col.push(ac[i].1);
        // }
        if ac[i].0 < *min.entry(ac[i].1).or_insert(ac[i].0) {
            *min.entry(ac[i].1).or_insert(ac[i].0) = ac[i].0;
        }
    }
    let mut max = 0;
    // for &j in col.iter() {
    for e in min.iter() {
        // println!("j={j}");
        max = max.max(*e.1);
    }
    println!("{}", max);
}
