use proconio::input;
use rustc_hash::FxHashSet;

fn main() {
    input! {
        n: usize,
        // a: [u32;n],
    };
    let mut set = FxHashSet::default();
    for t in 0..n {
        input! {
            l: usize,
            a: [u32;l],
        };
        set.insert(a);
    }
    println!("{}", set.len());
}
