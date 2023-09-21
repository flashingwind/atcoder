use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars;h],
    };

    let pat = ['s', 'n', 'u', 'k', 'e'];
    for i in 0..h {
        for j in 0..w {
            if j + 4 < w {
                let mut is_matched = true;
                for k in 0..5 {
                    if s[i][j + k] != pat[k] {
                        is_matched = false;
                        break;
                    }
                }
                if is_matched {
                    printij(i as isize, j as isize, 0, 1);
                    return;
                }
            }
            if j + 4 < w {
                let mut is_matched = true;
                for k in 0..5 {
                    if s[i][j + 4 - k] != pat[k] {
                        is_matched = false;
                        break;
                    }
                }
                if is_matched {
                    printij(i as isize, 4 + j as isize, 0, -1);
                    return;
                }
            }

            if h <= i + 4 {
                let mut is_matched = true;
                for k in 0..5 {
                    if s[i + k][j] != pat[k] {
                        is_matched = false;
                        break;
                    }
                }
                if is_matched {
                    printij(i as isize, j as isize, 1, 0);
                    return;
                }
            }
            if i + 4 < h {
                let mut is_matched = true;
                for k in 0..5 {
                    if s[i + 4 - k][j] != pat[k] {
                        is_matched = false;
                        break;
                    }
                }
                if is_matched {
                    printij(4 + i as isize, j as isize, -1, 0);
                    return;
                }
            }
            if i + 4 < h && j + 4 < w {
                let mut is_matched = true;
                for k in 0..5 {
                    if s[i + k][j + k] != pat[k] {
                        is_matched = false;
                        break;
                    }
                }
                if is_matched {
                    printij(i as isize, j as isize, 1, 1);
                    return;
                }
            }
            if i + 4 < h && j + 4 < w {
                let mut is_matched = true;
                for k in 0..5 {
                    if s[i + 4 - k][j + 4 - k] != pat[k] {
                        is_matched = false;
                        break;
                    }
                }
                if is_matched {
                    printij(4 + i as isize, 4 + j as isize, -1, -1);
                    return;
                }
            }
        }
    }
}

fn printij(i: isize, j: isize, di: isize, dj: isize) {
    // println!("printij:{i} {j} {di} {dj}");
    for k in 0..5 {
        // println!(
        //     "{} {} {}",
        //     i + k * di + 1,
        //     j + k * dj + 1,
        //     s[(i + k * di) as usize][(j + k * dj) as usize]
        // );
        println!("{} {}", i + k * di + 1, j + k * dj + 1,);
    }
}

fn is_matched(i: isize, j: isize, di: isize, dj: isize) {
    let pat = ['s', 'n', 'u', 'k', 'e'];
    if i + 4 < h && j + 4 < w {
        let mut is_matched = true;
        for k in 0..5 {
            if s[i + 4 - k][j + 4 - k] != pat[k] {
                is_matched = false;
                break;
            }
        }
        if is_matched {
            printij(4 + i as isize, 4 + j as isize, -1, -1);
            return;
        }
    }
}
