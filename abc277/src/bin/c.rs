use proconio::{input, marker::Usize1};
use rustc_hash::FxHashMap;

fn main() {
    input! {
        n: usize,
        mut raw_edges: [(Usize1,Usize1);n],
    };
    let mut edges = FxHashMap::default();
    for &(a, b) in raw_edges.iter() {
        edges.entry(a).or_insert(vec![]).push(b);
        edges.entry(b).or_insert(vec![]).push(a);
    }
    let mut max = 0;
    let mut q = vec![0usize];
    while let Some(a) = q.pop() {
        let mut bs = edges.entry(a).or_insert(vec![]);
        while let Some(b) = bs.pop() {
            max = max.max(b);
            q.push(b);
            // println!("{a}->{b}");
        }
    }

    println!("{}", max + 1);
}
