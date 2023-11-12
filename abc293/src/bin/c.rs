use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[u32;w];h],
    };
    let mut sets = vec![vec![HashSet::new(); w]; h];
    sets[0][0] = HashSet::from([a[0][0]]);
    for i in 1..h {
        if !sets[i - 1][0].contains(&a[i][0]) {
            sets[i][0] = sets[i - 1][0].clone();
            sets[i][0].insert(a[i][0]);
        } else {
            break;
        }
    }
    for i in 1..h {
        for j in 1..w {
            if !sets[i - 1][j].contains(&a[i][j]) {
                sets[i][j] = sets[i - 1][j].clone();
                sets[i][j].insert(a[i][j]);
            }
            if !sets[i][j - 1].contains(&a[i][j - 1]) {
                sets[i][j] = sets[i][j - 1].clone();
                sets[i][j].insert(a[i][j]);
            }
            if (&sets[i - 1][j] - &sets[i][j - 1]).len()
                <= (&sets[i][j - 1] - &sets[i - 1][j]).len()
            {
                sets[i][j] = sets[i][j - 1].clone();
            } else {
                sets[i][j] = sets[i - 1][j].clone();
            }
        }
    }
}
