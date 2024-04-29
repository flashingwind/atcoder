use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut cnt = 0;
    let mut cnt_same = 0;
    for pat in (0..s.len()).combinations(2) {
        let (i, j) = (pat[0], pat[1]);
        // }
        // for i in 0..s.len() {
        // for j in i + 1..s.len() {
        if s[i] != s[j] {
            cnt += 1;
        } else {
            cnt_same += 1;
        }

        // }
    }
    if cnt_same != 0 {
        cnt += 1;
    }
    println!("{}", cnt);
}
