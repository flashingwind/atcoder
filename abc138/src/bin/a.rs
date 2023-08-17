use std::println;

use proconio::input;

fn main() {
    input! {
        a: u32,
        s: String,
    };
    if 3200 <= a {
        println!("{}", s);
    } else {
        println!("red");
    }
}
