use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
    };
    let d: Vec<(i64, i64)> = vec![(1, 0), (0, 1), (h as i64 - 1, 0), (0, w as i64 - 1)];
    let mut map = vec![vec!['.'; w]; h];
    let mut dir = 2;
    let mut p = (0, 0);
    for _ in 0..n {
        if map[p.0][p.1] == '.' {
            map[p.0][p.1] = '#';
            dir = (dir + 3) % 4;
        } else {
            map[p.0][p.1] = '.';
            dir = (dir + 1) % 4;
        }
        p = (
            ((p.0 as i64 + d[dir].0) as usize) % h,
            ((p.1 as i64 + d[dir].1) as usize) % w,
        );
        // for i in 0..h {
        //     for j in 0..w {
        //         print!("{}", map[i][j]);
        //     }
        //     println!();
        // }
    }
    for i in 0..h {
        for j in 0..w {
            print!("{}", map[i][j]);
        }
        println!();
    }
}
