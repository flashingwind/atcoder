use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut b = Vec::new();
    for _ in 0..n {
        input! {
            c: usize,
            a: [usize;c],
        };
        b.push(a);
    }
    input! {
        x: usize,
    };
    let mut cnt_min = usize::MAX;
    let mut cnt_min_i = Vec::new();
    for (_, bb) in b.iter().enumerate() {
        if bb.contains(&x) && bb.len() < cnt_min {
            cnt_min = bb.len();
        }
    }
    for (i, bb) in b.iter().enumerate() {
        if bb.contains(&x) && bb.len() == cnt_min {
            cnt_min_i.push(i + 1);
        }
    }
    cnt_min_i.sort();
    println!("{}", cnt_min_i.len());
    println!("{}", cnt_min_i.iter().join(" "));
}
