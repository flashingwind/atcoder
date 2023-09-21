use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        ss: [String;3],
    };
    let dic: HashSet<String> = vec!["ABC", "ARC", "AGC", "AHC"]
        .into_iter()
        .map(String::from)
        .collect();

    for w in dic.iter() {
        if !ss.contains(&w.to_string()) {
            println!("{}", w);
        }
    }
}
