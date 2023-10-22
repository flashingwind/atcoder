use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [u32;n],
    };
    let mut last_q = vec![0u32; n];
    for t in 2..n {
        let mut q = p.clone();
        for pat in (n - t..n).permutations(t) {
            println!("{}..{}: {:?}", n - t, n, pat);
            let mut j = pat.len();
            for &i in pat.iter() {
                q[j] = p[i];
                j += 1;
            }
            // println!("{t}: {}", q.iter().map(|v| v.to_string()).join(" "));
            let mut is_ok = true;
            for i in 0..n {
                if q[i] != p[i] {
                    is_ok = false;
                    break;
                }
            }
            if is_ok {
                println!("OK: {}", last_q.iter().map(|v| v.to_string()).join(" "));
                return;
            }
            last_q = q.clone();
        }
    }
}
