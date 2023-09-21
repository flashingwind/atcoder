use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        _: usize,
        s: [Chars;h],
    };
    let mut cnt = 0;
    for l in s.iter() {
        for c in l.iter() {
            if *c == '#' {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}
