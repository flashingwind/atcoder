use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    };
    /*
    let mut map = vec![vec![false; h]; w];
    let mut is_changed = true;
    map[0][0] = true;
    while is_changed {
        is_changed = false;
        for (i, row) in map.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                // check (i,j)
                if map[i][j] {
                    if 0 < i - 1 {
                        if 0 < j - 1 {
                            map[i - 1][j - 1] = true;
                            is_changed = true;
                        }
                        if j + 1 < w {
                            map[i - 1][j + 1] = true;
                            is_changed = true;
                        }
                    } else if i + 1 < h {
                        if 0 < j - 1 {
                            map[i + 1][j - 1] = true;
                            is_changed = true;
                        }
                        if j + 1 < w {
                            map[i + 1][j + 1] = true;
                            is_changed = true;
                        }
                    }
                }
            }
        }
    } */
    if w == 1 || h == 1 {
        println!("1");
        return;
    }
    let hh = h / 2;
    let ww = w / 2;
    let mut s = hh * ww * 2;
    if h % 2 != 0 && w % 2 != 0 {
        s += hh + ww + 1;
    } else if h % 2 != 0 && w % 2 == 0 {
        s += ww;
    } else if h % 2 == 0 && w % 2 != 0 {
        s += hh;
    }
    // println!("hh={hh},ww={ww}={}", s);
    println!("{}", s);
}
