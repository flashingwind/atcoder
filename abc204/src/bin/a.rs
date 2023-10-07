use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        x:u32,
        y:u32,
    };
    if x == y {
        println!("{}", x);
    } else {
        let mut vec = HashSet::from([0, 1, 2]);
        vec.remove(&x);
        vec.remove(&y);
        println!("{}", vec.iter().next().unwrap());
    };
}
