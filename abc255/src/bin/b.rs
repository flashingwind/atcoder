use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        ls_idx: [Usize1;k],
        ps: [(isize,isize);n]
    };
    let mut obj = ps.clone();
    for i in ls_idx.iter().rev() {
        obj.remove(*i);
    }
    println!("{:?}", obj);
    let mut max = 0;
    for &i in ls_idx.iter() {
        for &(x, y) in obj.iter() {
            let d = ps[i].0.abs_diff(x).pow(2) + ps[i].1.abs_diff(y).pow(2);
            max = max.max(d);
        }
    }
    println!("{}", (max as f64).sqrt());
}
