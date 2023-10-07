use proconio::{input, marker::Chars};

fn main() {
    let dd = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (-1, 0),
        (1, 1),
    ];
    input! {
        h: usize,
        w: usize,
        bw: [Chars;h],
    };
    let mut color = vec![vec![false; w]; h];
    for i in 0..j {
        for j in 0..w {
            color[i][j] = if bw[i][j] == "#" { true } else { false };
        }
    }
    let mut is_visited = vec![vec![false; w]; h];
    let mut is_reachable = vec![vec![false; w]; h];
    let mut inbound_cnt = vec![vec![0usize; w]; h];
    let mut is_changed = false;
    loop {
        for i in 0..h {
            for j in 0..w {
                if i == 0 && j == 0 {
                    continue;
                }
                if !is_visited[i][j] {
                    let cnt = 0;
                    for (di, dj) in dd.iter() {
                        if 0 <= i + di && i + di < n && 0 <= i + di && i + di < n {
                            if is_reachable[i + di][j + dj] && color[i + di][j + dj] != color[i][j]
                            {
                                inbound_cnt[i][j] += inbound_cnt[i + di][j + dj];
                                is_reachable[i][j] = true;
                                is_visited[i][j] = true;
                                is_changed = true;
                            }
                        }
                    }
                }
            }
        }
        if !is_changed {
            break;
        }
    }
}
