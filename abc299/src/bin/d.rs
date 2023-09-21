use std::io::{self, BufReader};

use proconio::{input, source::line::LineSource};
fn main() {
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
    input! {
        n: usize,
    };
    let mut p = 1;
    let mut is_ok = false;
    let mut s_p = 0;
    while !is_ok {
        println!("? {}", p + 1);
        input! {
            s_p_1: usize,
        };
        if s_p_1 != s_p {
            println!("! {}", p);
            return;
        }
        s_p = s;
        p += 1;
    }
}
