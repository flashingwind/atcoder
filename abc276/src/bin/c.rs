use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize;n],
    };
    for pat in (1..=n).permutations(n) {
        let is_eq
        for (i, &t) in pat.iter().enumerate() {
            if p[i]!=t{
                break;
            }
        }
    }
}
