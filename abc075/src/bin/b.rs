use std::char::from_digit;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut s: [Chars;h],
    };
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                let mut cnt = 0;
                if 1 <= i {
                    if 1 <= j && s[i - 1][j - 1] == '#' {
                        cnt += 1;
                    }
                    if s[i - 1][j] == '#' {
                        cnt += 1;
                    }
                    if j < w - 1 && s[i - 1][j + 1] == '#' {
                        cnt += 1;
                    }
                }

                if 1 <= j && s[i][j - 1] == '#' {
                    cnt += 1;
                }
                if s[i][j] == '#' {
                    cnt += 1;
                }
                if j < w - 1 && s[i][j + 1] == '#' {
                    cnt += 1;
                }

                if i < h - 1 {
                    if 1 <= j && s[i + 1][j - 1] == '#' {
                        cnt += 1;
                    }
                    if s[i + 1][j] == '#' {
                        cnt += 1;
                    }
                    if j < w - 1 && s[i + 1][j + 1] == '#' {
                        cnt += 1;
                    }
                }
                s[i][j] = from_digit(cnt, 10).unwrap();
            }
        }
    }
    for l in s.iter() {
        for c in l.iter() {
            print!("{}", c);
        }
        println!();
    }
}
