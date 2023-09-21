use proconio::{input, marker::Chars};

#[allow(clippy::manual_swap)]
fn main() {
    input! {
        mut s: Chars,
        a: usize,
        b: usize,
    };
    let tmp = s[a - 1];
    s[a - 1] = s[b - 1];
    s[b - 1] = tmp;
    for c in s.iter() {
        print!("{}", *c);
    }
    println!();
}
