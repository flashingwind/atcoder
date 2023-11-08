use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32;n],
    };
    let mut min_xor;
    {
        //区切らない
        let mut or = 0;
        // println!("0~{}, ", n - 1);
        for i in 0..n {
            // r = tail;
            // v.push(l);
            or |= a[i];
        }
        min_xor = or;
    }
    for cnt in 1..=n {
        if n < 2 {
            println!("{}", min_xor);
            return;
        }
        for pat_raw in (0..=n - 2).combinations(cnt) {
            let mut pat = pat_raw.clone();
            let mut xor = 0;
            let mut last_i = 0;
            pat.push(n - 1);
            // println!("{:?}", pat_raw);
            for cut in pat.iter() {
                let mut or = 0;
                // print!("{last_i}~{cut}, ");
                for i in last_i..=*cut {
                    or |= a[i];
                    last_i = cut + 1;
                }
                xor ^= or;
            }
            // println!();
            min_xor = min_xor.min(xor);
        }
    }
    println!("{}", min_xor);
}
