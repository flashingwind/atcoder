use proconio::{input, source::line::LineSource};
use std::io::{self, BufReader};

fn main() {
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
    input! {
        n: usize,
    };
    let mut vec_is_used = vec![false; 2 * n + 2];
    loop {
        for (m, is_used) in vec_is_used.iter_mut().enumerate().skip(1) {
            if !*is_used {
                println!("{}", m);
                *is_used = true;
                break;
            }
        }
        input! {
            m2: usize,
        };
        vec_is_used[m2] = true;
        if m2 == 0 {
            return;
        }
        // println!("{:?}", vec_is_used);
    }
}
