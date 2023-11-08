use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s_input: Chars,
    };
    s_input.sort();
    let s = s_input.iter().map(|v| *v).unique().collect_vec();
    // println!("{:?}", s);
    if s.contains(&'W') ^ s.contains(&'E') {
        println!("No");
        return;
    }
    if s.contains(&'N') ^ s.contains(&'S') {
        println!("No");
        return;
    }
    println!("Yes");
}
