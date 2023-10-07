use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut x: u32,
        s: Chars
    };
    for &c in s.iter() {
        if c == 'x' && x != 0 {
            x -= 1;
        } else if c == 'o' {
            x += 1;
        }
    }
    println!("{}", x);
}
