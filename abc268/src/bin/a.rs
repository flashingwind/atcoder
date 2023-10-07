use itertools::Itertools;
use proconio::input;

fn main() {
    let n = 5;
    input! {
        mut a: [u32;n],
    };
    a.sort();
    println!("{}", a.iter().unique().count());
}
