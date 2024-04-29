use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        map: [Chars;h],
    };
    let w = map[0].len();
    let d: Vec<(i64, i64)> = vec![(1, 0), (0, 1), (h as i64 - 1, 0), (0, w as i64 - 1)];
    let mut map = vec![vec!['.'; w]; h];
    let mut dir = 2;
    let mut dist = vec![vec![i64::MAX; w]; h];
    let mut is_filled = false;
    while is_filled {
        for y in 0..h as i64 {
            for x in 0..w as i64 {
                if map[y][x] == 'P' {
                    dist[y][x] = 0;
                    for &(d0, d1) in d.iter() {
                        let yy = y + d0;
                        let xx = x + d1;
                        if 0 <= yy && yy < h as i64 && 0 <= xx && xx < w as i64 {
                        } else {
                            if dist[yy as usize][xx as usize] == i64::MAX {
                                dist[yy as usize][xx as usize] = dist[y as usize][x as usize] + 1;
                            }
                        }
                    }
                }
            }
        }
    }
}
