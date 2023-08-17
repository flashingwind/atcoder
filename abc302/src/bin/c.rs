use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut ss: [Chars;n],
    };

    let mut r = vec![];
    for i1 in 0..n {
        for i2 in i1 + 1..n {
            let mut diff = 0;
            for j in 0..m {
                if ss[i1][j] != ss[i2][j] {
                    diff += 1;
                }
            }
            if diff == 1 {
                r.push((i1, i2));
            }
        }
    }
    // println!("r={:?}", r);
    // // ハミルトン閉路
    // let mut deg = vec![0; n];
    // for &(p1, p2) in r.iter() {
    //     deg[p1] += 1;
    //     deg[p2] += 1;
    // }
    // println!("deg={:?}", deg);
    // for i1 in 0..n {
    //     if deg[i1] == 0 {
    //         println!("No");
    //         return;
    //     }
    //     for i2 in i1 + 1..n {
    //         if deg[i2] == 0 || !r.contains(&(i1, i2)) && n < deg[i1] + deg[i2] {
    //             println!("No");
    //             return;
    //         }
    //     }
    // }
    // println!("Yes");

    for pat in (0..n).permutations(n) {
        let mut i1 = pat[0];
        let mut is_ok = true;
        for &i2 in pat.iter().skip(1) {
            if !r.contains(&(i1, i2)) && !r.contains(&(i2, i1)) {
                is_ok = false;
                break;
            }
            i1 = i2;
        }
        if is_ok {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
