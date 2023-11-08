use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        mut a: [Usize1;k],
        l: [Usize1;q],
    };
    // let map = vec![false; n + 1];
    // for ai in a.iter() {
    //     map[ai] = true;
    // }
    for x in 0..q {
        let i = l[x];
        if a[i] == n - 1 {
            continue;
        }
        if i == k - 1 && a[i] < n - 1 {
            a[i] += 1;
        } else if a[i] + 1 != a[i + 1] {
            a[i] += 1;
        }
    }
    println!("{}", a.iter().map(|v| *v + 1).join(" "));
}
