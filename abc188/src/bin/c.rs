use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32;2usize.pow(n as u32)],
    };
    // let a1 = a[0..a.len() / 2].to_vec();
    // let a2 = a[a.len() / 2..a.len()].to_vec();
    // println!("{:?}", a1);
    // println!("{:?}", a2);
    let first_half = a[0..a.len() / 2].iter().position_max().unwrap();
    let second_half = a[a.len() / 2..a.len()].iter().position_max().unwrap() + a.len() / 2;
    // println!("a[first_half]=a[{}]={}", first_half + 1, a[first_half]);
    // println!("a[second_half]=a[{}]={}", second_half + 1, a[second_half]);
    if a[first_half] < a[second_half] {
        println!("{}", first_half + 1);
    } else {
        println!("{}", second_half + 1);
    }
}
