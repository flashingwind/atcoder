use std::{collections::HashSet, iter};

use proconio::input;

fn main() {
    input! {
        a: [[u32;9];9],
    };
    for i in 0..9 {
        let checked: HashSet<_> = a[i].iter().collect();
        if checked.len() != 9 {
            println!("No");
            return;
        }
    }
    let mut checked = vec![HashSet::new(); 9];
    for i in 0..9 {
        for j in 0..9 {
            checked[j].insert(a[i][j]);
        }
    }
    for j in 0..9 {
        if checked[j].len() != 9 {
            println!("No");
            return;
        }
    }
    let dir = [
        (0, 0),
        (1, 0),
        (2, 0),
        (0, 1),
        (1, 1),
        (2, 1),
        (0, 2),
        (1, 2),
        (2, 2),
    ];
    for i_raw in [0, 3, 6].iter() {
        for j_raw in [0, 3, 6].iter() {
            let mut checked = HashSet::new();
            for (di, dj) in dir.iter() {
                let i = i_raw + di;
                let j = j_raw + dj;
                checked.insert(a[i][j]);
            }
            if checked.len() != 9 {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
