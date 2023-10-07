use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};
use rustc_hash::FxHashMap;

fn main() {
    input! {
        n: usize,
        ab: [(Usize1,Usize1);n],
    };
    let mut sum = 0;
    for &ai in ab.iter() {
        if 10 < ai {
            sum += abi - 10;
        }
    }
    let mut uf = UnionFind::new(n);
    let mut set = FxHashMap::default();

    for i in 0..m {
        set.entry((ab[i].0, ab[i].1)).or_insert(vec![]).push(i);
        uf.union(ab[i].0, ab[i].1);
    }
    println!("{}", sum);
}
