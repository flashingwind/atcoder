use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };
    let mid: String = s[1..=s.len() - 2].iter().unique().collect();
    if s[0] == '<' && mid == "=" && *s.last().unwrap() == '>' {
        println!("Yes");
    } else {
        println!("No");
    }
}
