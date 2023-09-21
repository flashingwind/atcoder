use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut map: [Chars;h],
    };
    map.insert(0, Vec::new());
    for m in map.iter_mut() {
        m.insert(0, 'X');
    }
    let mut i = 1;
    let mut j = 1;
    let mut is_checked = vec![vec![false; w + 1]; h + 1];
    is_checked[i][j] = true;
    loop {
        // print!("({i},{j})-");
        if map[i][j] == 'U' && i == 1 {
            break;
        } else if map[i][j] == 'U' && i != 1 {
            i -= 1;
        } else if map[i][j] == 'D' && i == h {
            break;
        } else if map[i][j] == 'D' && i != h {
            i += 1;
        } else if map[i][j] == 'L' && j == 1 {
            break;
        } else if map[i][j] == 'L' && j != 1 {
            j -= 1;
        } else if map[i][j] == 'R' && j == w {
            break;
        } else if map[i][j] == 'R' && j != w {
            j += 1;
        }
        if is_checked[i][j] {
            println!("-1");
            return;
        }
        is_checked[i][j] = true;
    }
    // println!();
    println!("{} {}", i, j);
}
