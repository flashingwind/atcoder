use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [u32;n],
        q: [u32;n],
    };
    let str_p: String = p.iter().map(|c| c.to_string()).collect();
    let str_q: String = q.iter().map(|c| c.to_string()).collect();
    let mut a = 0;
    let mut b = 0;
    for (i, pat) in (1..=n).permutations(n).enumerate() {
        let str1: String = pat.iter().map(|c| c.to_string()).collect();
        if str_p == str1 {
            a = i;
        }
        if str_q == str1 {
            b = i;
        }
    }
    if a < b {
        println!("{}", b - a);
    } else {
        println!("{}", a - b);
    }
}
