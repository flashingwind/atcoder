use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        arr: [usize;n],
    };
    let mut is_first = true;
    for (i, a) in arr.iter().enumerate().sorted_by(|a, b| a.1.cmp(&b.1)) {
        if is_first {
            is_first = false;
            print!("{}", i + 1);
        } else {
            print!(" {}", i + 1);
        }
    }
    println!();
}
