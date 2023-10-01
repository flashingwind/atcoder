use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        t: u32,
        r: [(u32,u32);n],
    };

    for &(c, ti) in r.iter().sorted() {
        if ti <= t {
            // println!("{i}:{c},{_ti}");
            println!("{}", c);
            return;
        }
    }
    println!("TLE");
}
