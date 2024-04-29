use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[char;w];h],
        n: usize,
        drug: [(Usize1,Usize1,u32);n]
    };
    let dirs: Vec<(isize, isize)> = vec![(0, -1), (-1, 0), (1, 0), (0, 1)];
    let mut map = vec![vec![u32::MAX; w]; h];
    let mut d_map = vec![vec![0; w]; h];
    for d in drug.iter() {
        d_map[d.0][d.1] = d.2;
    }
    let mut eng = 0;
    let mut mincost = u64::MAX;
    for i in 0..h {
        for j in 0..w {
            for dir in dirs.iter() {
                if -dir.0 > i as isize {
                    break;
                }
                if -dir.1 > j as isize {
                    break;
                }
                let ii = (i as isize + dir.0) as usize;
                let jj = (j as isize + dir.1) as usize;
                if ii < h && jj < w {}
                if a[ii][jj] == 'S' {
                    map[i][j] = 1;
                } else if a[ii][jj] == 'T' {
                    map[ii][jj] = map[ii][jj].min(map[i][j] + 1);
                    println!("{}", map[ii][jj]);
                }else if d_map[ii][jj]!=0 {

                }
                } else {
                    map[ii][jj] = map[ii][jj].min(map[i][j] + 1);
                }
            }
        }
    }
}
