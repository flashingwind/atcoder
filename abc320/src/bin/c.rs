use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        m: usize,
        s: [Chars;3],
    }

    let mut min_t = std::usize::MAX;

    for xx in (0usize..3).permutations(3) {
        let x = xx[0];
        let y = xx[1];
        let z = xx[2];
        // println!("{x}-{y}-{z}");
        for i in 0..m {
            for jj in (i + 1)..(i + 1 + m) {
                let j = jj % m;
                if s[x][i] == s[y][j] {
                    for kk in (jj + 1)..(jj + 1 + m) {
                        let k = kk % m;
                        if s[x][i] == s[z][k] {
                            // println!("{i}:{} {jj}:{} {kk}:{}", s[x][i], s[y][j], s[z][k]);
                            if kk < min_t {
                                min_t = kk;
                            }
                            break;
                        }
                    }
                }
            }
        }
    }
    if min_t == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", min_t);
    }
}
