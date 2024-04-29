use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        t: [Usize1;q],
    };
    let mut te = vec![1; n];
    for &tt in t.iter() {
        if te[tt] == 1 {
            te[tt] = 0;
        } else {
            te[tt] = 1;
        }
    }
    println!("{}", te.iter().sum::<usize>());
}
