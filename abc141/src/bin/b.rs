use maplit::hashset;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let odd_set = hashset! { 'R', 'U', 'D' };
    let eve_set = hashset! { 'L', 'U', 'D' };
    for (i, c) in s.iter().enumerate() {
        if i % 2 == 1 && !eve_set.contains(c) {
            println!("No");
            return;
        } else if i % 2 == 0 && !odd_set.contains(c) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
