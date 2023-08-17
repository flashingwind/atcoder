use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        t: Chars,
    };
    let mut xy = (0, 0);
    let mut direct = (1, 0);
    for c in t {
        if c == 'S' {
            // move 1 step
            xy = (xy.0 + direct.0, xy.1 + direct.1);
        } else if c == 'R' {
            // rot
            if direct == (1, 0) {
                direct = (0, -1);
            } else if direct == (0, -1) {
                direct = (-1, 0);
            } else if direct == (-1, 0) {
                direct = (0, 1);
            } else if direct == (0, 1) {
                direct = (1, 0);
            }
        }
    }
    println!("{} {}", xy.0, xy.1);
}
