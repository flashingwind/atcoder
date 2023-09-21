use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };
    let n = s.len();
    s.insert(0, '0');
    let s1: Vec<char> = s.get(1..=(n - 1) / 2).unwrap().to_vec();
    let s1_rev: Vec<char> = s1.iter().rev().copied().collect();
    let s2: Vec<char> = s.get((n + 3) / 2..=n).unwrap().to_vec();
    let s2_rev: Vec<char> = s2.iter().rev().copied().collect();
    // println!("{:?}", s1);
    // println!("{:?}", s1_rev);
    // println!("{:?}", s2);
    // println!("{:?}", s2_rev);
    if s1 == s2_rev && s1 == s1_rev && s2 == s2_rev {
        println!("Yes");
    } else {
        println!("No");
    }
}
