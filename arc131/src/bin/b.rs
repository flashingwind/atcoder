use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut c: [Chars;h],
    };
    let ds = vec![(-1_isize, -1_isize), (0, -1), (-1, 0), (1, 0), (0, 1)];
    let allcol = HashSet::from([1, 2, 3, 4, 5]);
    for i in 0..h {
        for j in 0..w {
            let mut colors = HashSet::new();
            for &d in ds.iter() {
                let (ii, jj) = (i as isize + d.0, j as isize + d.1);
                if 0 <= ii && ii < h as isize && 0 <= jj && jj < w as isize {
                    if c[ii as usize][jj as usize] == '.' {
                    } else {
                        colors.insert(c[ii as usize][jj as usize].to_digit(10).unwrap());
                    }
                }
            }
            let diffcol = &allcol ^ &colors;
            for d in &diffcol {
                c[i][j] = d.to_string().chars().nth(0).unwrap();
                break;
            }
        }
    }
    for cc in c.iter() {
        for ccc in cc.iter() {
            print!("{}", *ccc);
        }
        println!();
    }
}
