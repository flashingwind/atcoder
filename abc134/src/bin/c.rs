use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32;n],
    };
    let mut max = *a.iter().max().unwrap();
    let mut max_i = a.iter().position_max().unwrap();
    a[max_i] = 1;
    for i in 0..n {
        if i == max_i {
            println!("{}", a.iter().max().unwrap());
        } else {
            println!("{}", max);
        }
    }
}
