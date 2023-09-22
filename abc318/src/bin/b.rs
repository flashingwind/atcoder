use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [((usize,usize),(usize,usize));n],
    }
    let mut map = vec![vec![false; 101]; 101];
    let mut x_min = 0;
    let mut x_max = 0;
    let mut y_min = 0;
    let mut y_max = 0;
    let mut s = 0usize;
    for &(x, y) in a.iter() {
        let mut ss = 0;
        for xx in x.0..x.1 {
            for yy in y.0..y.1 {
                if !map[xx][yy] {
                    map[xx][yy] = true;
                    ss += 1;
                }
            }
        }
        s += ss;
        // println!("{}-{},{}-{}: {s}+{ss}", x.0, x.1, y.0, y.1);
    }
    println!("{}", s);
}
