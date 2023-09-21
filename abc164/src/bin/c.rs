use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String;n],
    };
    let uniq: HashSet<String> = s.into_iter().collect();
    println!("{}", uniq.len());
}
