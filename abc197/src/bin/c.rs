use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32;n],
    };
    let mut min_xor = u32::MAX;
    for t in 1..=n - 1 {
        for pat in (0..n - 1).combinations(t) {
            let mut xor = 0;
            let mut l = 0;
            for &r in pat.iter() {
                let mut or = 0;
                for i in l..=r {
                    or |= a[i];
                }
                // println!("    {l}..={r}: or={or}");
                xor ^= or;
                l = r;
            }
            let mut or = 0;
            for i in l..n {
                or |= a[i];
            }
            // println!("    {l}..{n}: or={or}");
            xor ^= or;
            // println!("pat:{:?} xor={xor}", pat);
            min_xor = min_xor.min(xor);
        }
    }
    println!("{}", min_xor);
}
