use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        ls_idx: [Usize1;k],
        ps: [(isize,isize);n]
    };
    let mut max = 0;
    for &i in ls_idx.iter() {
        for &(x, y) in ps.iter() {
            let d = (x - ps[i].0).pow(2) + (y - ps[i].1).pow(2);
            max = max.max(d);
        }
    }
    println!("{}", (max as f64).sqrt());
}
