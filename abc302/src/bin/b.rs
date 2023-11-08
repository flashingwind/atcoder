use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars;h],
    };

    let pat = ['s', 'n', 'u', 'k', 'e'];
    let d = vec![
        (-1, 0),
        (0, -1),
        (-1, -1),
        (1, -1),
        (-1, 1),
        (1, 0),
        (0, 1),
        (1, 1),
    ];
    for i in 0..h as isize {
        for j in 0..w as isize {
            for &(di, dj) in d.iter() {
                // println!("{i},{j}-->{di},{dj}:");
                let mut log = vec![];
                let mut is_matched = true;
                for k in 0..5 {
                    let ii = i + di * k;
                    let jj = j + dj * k;
                    if 0 <= ii && 0 <= jj && ii < h as isize && jj < w as isize {
                        if s[ii as usize][jj as usize] != pat[k as usize] {
                            is_matched = false;
                            break;
                        } else {
                            // println!("    {},{}:{}", ii, jj, s[ii as usize][jj as usize]);
                        }
                    } else {
                        is_matched = false;
                        break;
                    }
                    log.push((ii, jj));
                }
                if is_matched {
                    for (r, c) in log.iter() {
                        println!("{} {}", r + 1, c + 1);
                    }
                    return;
                }
            }
        }
    }
}
