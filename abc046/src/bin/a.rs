use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        mut a: [u32;3],
    };
    a.sort();
    println!("{}", a.iter().unique().count());
}
