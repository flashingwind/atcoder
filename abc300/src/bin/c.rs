use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        map0: [Chars;h],
    };
    let mut ans = vec![0; h.min(w)];
    let mut map = vec![vec![0_i32; w]; w];
    for y in 0..h {
        for x in 0..w {
            if map0[x][y] == '#' {
                map[x][y] = -1;
            }
        }
    }
    let mut id = 1;
    for y in 0..h {
        for x in 0..w {
            if map[x][y] == -1 {
                let mut size = 0;
                for x2 in x + 1..w {
                    size += 1;
                    if map[x][y] == -1 {
                        map[x][y] = id;
                        break;
                    }
                    map[x][y] = id;
                }
            }
        }
    }
}
