use num::integer::Roots;
use proconio::input;

fn main() {
    input! {
        n: u32,
    };
    println!("{}", n.sqrt().pow(2));
}
