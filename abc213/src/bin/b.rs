use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32;n],
    };
    let mut b = a.iter().enumerate().collect_vec();
    b.sort_by(|&v1, &v2| v1.1.cmp(v2.1));
    // println!("{:?}", b);
    println!("{}", b[b.len() - 2].0 + 1);
}
