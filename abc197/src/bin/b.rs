use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        h: usize,
        w: usize,
        x: Usize1,
        y: Usize1,
        map: [Chars;h],
    };
    let mut cnt = 1;
    for i in (0..x).rev() {
        if map[i][y] == '#' {
            // println!("↑{i},{y}:break");
            break;
        } else {
            cnt += 1;
        }
    }
    for i in x + 1..h {
        if map[i][y] == '#' {
            // println!("↓{i},{y}:break");
            break;
        } else {
            cnt += 1;
        }
    }
    for j in (0..y).rev() {
        if map[x][j] == '#' {
            // println!("←{x},{j}:break");
            break;
        } else {
            cnt += 1;
        }
    }
    for j in y + 1..w {
        if map[x][j] == '#' {
            // println!("→{x},{j}:break");
            break;
        } else {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
