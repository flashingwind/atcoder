use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    for pat in (1..=m).combinations(n).sorted() {
        println!("{}", pat.iter().map(|v| v.to_string()).join(" "));
    }
}
