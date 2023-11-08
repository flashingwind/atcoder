use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        n: usize,
        po_vec: [(String,usize);n],
    };
    let mut set = FxHashMap::default();
    for i in (0..n).rev() {
        set.insert(po_vec[i].0.as_str(), [po_vec[i].1, i + 1]);
    }
    let mut rank = set.iter().collect::<Vec<_>>();
    // println!("{:?}", rank);
    rank.sort_by(|&a, &b| a.1[1].cmp(&b.1[1]));
    rank.sort_by(|&a, &b| b.1[0].cmp(&a.1[0]));
    // println!("{:?}", rank);
    println!("{}", rank[0].1[1]);
}
