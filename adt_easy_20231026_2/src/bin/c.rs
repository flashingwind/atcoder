use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        t: Chars,
    };
    let ds = vec![(1, 0), (0, -1), (-1, 0), (0, 1)];
    let mut d = 0;
    // let mut x = (0, 0);
    let mut x = 0;
    let mut y = 0;
    for &c in t.iter() {
        if c == 'S' {
            let (dx, dy) = ds[d];
            x += dx;
            y += dy;
        } else {
            d = (d + 1) % 4;
        }
    }
    println!("{} {}", x, y);
}
