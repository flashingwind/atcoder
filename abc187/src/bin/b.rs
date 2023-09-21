use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [(i64,i64);n],
    };
    let mut cnt = 0;
    for indices in (0..n).combinations(2) {
        let (i, j) = (indices[0], indices[1]);
        let (x1, y1) = p[i];
        let (x2, y2) = p[j];
        // y=ax+b, a=d/c
        if (y1 - y2).abs() <= (x1 - x2).abs() {
            cnt += 1;
            // println!("{i},{j}:{cnt}: c/d={}-{}/{}-{}={c}/{d}", y2, y1, x2, x1);
        }
    }
    println!("{}", cnt);
}
