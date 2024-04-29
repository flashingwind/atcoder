use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut cnt = 0;
    let mut cnt_same = 0;
    let mut cache = vec![-1_i64; s.len()];
    for i in (0..s.len()).rev() {
        if cache[i] != -1 {
            cnt += cache[i];
            continue;
        }
        for j in (i + 1..s.len()).rev() {
            if s[i] != s[j] {
                cnt += 1;
            } else {
                cache[j] = cnt;
                cnt_same += 1;
            }
        }
    }
    if cnt_same != 0 {
        cnt += 1;
    }
    println!("{}", cnt);
}
