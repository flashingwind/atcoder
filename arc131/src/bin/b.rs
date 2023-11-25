use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars;h],
    };
    let ds = vec![
        (-1_isize, -1_isize),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let mut is_changed;
    let mut all_colors = HashSet::new();
    let mut possible=vec![vec![HashSet::new();w];h];
    let
    ['1', '2', '3', '4', '5'].iter().map(|v| all_colors.insert(*v));
    loop {
        is_changed = false;

        for i in 0..h {
            for j in 0..w {
                let mut set = HashSet::from(
                    ds.iter()
                        .filter(|&&(di, dj)| (di.abs() as usize) < i && (dj.abs() as usize) < j)
                        .map(|&(di, dj)| c[(i as isize + di) as usize][(j as isize + dj) as usize])
                        .collect::<HashSet<char>>(),
                );
                possible[i][j] = &all_colors - &set;
                if diff.len()
            }
        }
        if !is_changed {
            break;
        }
    }
}
