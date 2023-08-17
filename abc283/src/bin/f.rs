use std::vec;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut p: [isize;n],
    };
    p.insert(0, 0);
    let mut min = vec![2 * n as isize + 1; n + 1];
    let mut d = vec![vec![2 * n as isize + 1; n + 1]; n + 1];
    for i in 1..=n as usize {
        for j in i + 1..=n {
            {
                let ii = i;
                let jj = j;
                d[ii][jj] = (p[ii] - p[jj]).abs() + (ii as isize - jj as isize).abs();
            }
            {
                let ii = j;
                let jj = i;
                d[ii][jj] = (p[ii] - p[jj]).abs() + (ii as isize - jj as isize).abs();
            }
        }
    }
    for i in 1..=n {
        for j in 1..=n {
            if d[i][j] < min[i] {
                min[i] = d[i][j];
            }
        }
    }
    // println!("d={:?}", d);
    // println!("min={:?}", min);
    print!("{}", min[1]);
    for m in min[2..=n].iter() {
        print!(" {}", *m);
    }
    println!();
}
