use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32;n],
    };
    println!("{}", a.iter().unique().count());
}
