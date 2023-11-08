use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
    };
    let aplph: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .to_string()
        .chars()
        .collect_vec();
    let mut v = vec![];
    for c in aplph.iter() {
        for t in 0..n {
            v.push(*c);
        }
    }
    println!("{}", v[x - 1]);
}
