use proconio::{input, marker::Usize1};

fn main() {
    input! {
        r: Usize1,
        l: Usize1,
    };
    let s = ["a", "t", "c", "o", "d", "e", "r"];
    println!("{}", s[r..=l].join(""));
}
