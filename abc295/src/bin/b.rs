use proconio::{input, marker::Chars};

fn main() {
    input! {
        r: usize,
        c: usize,
        b: [Chars;r],
    };
    let mut b2 = b.to_owned();
    for i in 0..r {
        for j in 0..c {
            if let Some(dd) = b[i][j].to_digit(10) {
                let d = dd as usize;
                for ii2 in (i as isize - d as isize)..=(i as isize + d as isize) {
                    for jj2 in (j as isize - d as isize)..=(j as isize + d as isize) {
                        if 0 <= ii2 && 0 <= jj2 && ii2 < r as isize && jj2 < c as isize {
                            let i2 = ii2 as usize;
                            let j2 = jj2 as usize;
                            if is_inside(i, j, i2, j2, d) {
                                b2[i2][j2] = '.';
                            }
                        }
                    }
                }
            }
        }
    }
    for i in 0..r {
        for j in 0..c {
            print!("{}", b2[i][j]);
        }
        println!();
    }
}

fn is_inside(i: usize, j: usize, i2: usize, j2: usize, d: usize) -> bool {
    i2 <= i && j2 <= j && i - i2 + j - j2 <= d
        || i2 <= i && j2 > j && i - i2 + j2 - j <= d
        || i2 > i && j2 <= j && i2 - i + j - j2 <= d
        || i2 > i && j2 > j && i2 - i + j2 - j <= d
}
