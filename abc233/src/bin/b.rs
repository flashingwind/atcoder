use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        l: Usize1,
        r: Usize1,
        s: Chars,
    };
    for i in 0..s.len() {
        if i < l || r < i {
            print!("{}", s[i]);
        } else {
            // inside
            print!("{}", s[r - (i - l)]);
        }
    }
    println!();
}
