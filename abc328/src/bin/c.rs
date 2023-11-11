use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        lr:[(Usize1,Usize1);q],
    };
    let mut sum = vec![0u32; n];
    for i in 0..n - 1 {
        if s[i] == s[i + 1] {
            sum[i] += 1;
            // println!("{i}: {}:{}={}:{} {}", i, s[i], i + 1, s[i + 1], sum[i]);
        } else {
            // println!("{i}: {}", sum[i]);
        }
        sum[i + 1] = sum[i];
    }
    for &(l, r) in lr.iter() {
        // println!("{},{}:", l, r);
        if r == 0 {
            println!("0");
        } else if l == 0 {
            println!("{}", sum[r - 1]);
        } else {
            println!("{}", sum[r - 1] - sum[l - 1]);
        }
    }
}
