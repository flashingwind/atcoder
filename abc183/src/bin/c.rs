use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u32,
        t: [[u32;n];n],
    };
    let mut cnt: u32 = 0;
    for pat in (1..n).permutations(n - 1) {
        let mut i = 0;
        let mut cost = 0;
        for j in pat.iter() {
            cost += t[i][*j];
            // println!("{i}->{j}: {} cost={cost}", t[i][*j]);
            i = *j;
        }
        cost += t[i][0];
        // println!("{i}->{0}: {} cost={cost}", t[i][0]);
        if cost == k {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
