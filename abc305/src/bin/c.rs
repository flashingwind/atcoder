use proconio::{input, marker::Chars};
use rustc_hash::FxHashMap;

fn main() {
    input! {
        h: usize,
        w: usize,
        m: [Chars;h],
    };
    let mut sq = FxHashMap::default();
    for y in 0..h {
        for x in 0..w {
            if m[y][x] == '#' {
                sq.entry(y)
                    .or_insert(FxHashMap::default())
                    .entry(x)
                    .or_insert(1usize);
            }
        }
    }
    let y_min = *sq.keys().min().unwrap();
    let y_max = *sq.keys().max().unwrap();
    let mut x_min = std::usize::MAX;
    let mut x_max = std::usize::MIN;
    for y in y_min..=y_max {
        let x_min_tmp = *sq.entry(y).or_default().keys().min().unwrap();
        if x_min_tmp < x_min {
            x_min = x_min_tmp;
        }
        let x_max_tmp = *sq.entry(y).or_default().keys().max().unwrap();
        if x_max < x_max_tmp {
            x_max = x_max_tmp;
        }
    }
    // println!("y: {} {}", y_min, y_max);
    // println!("x: {} {}", x_min, x_max);
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if let Some(_) = sq.entry(y).or_default().get(&x) {
            } else {
                println!("{} {}", y + 1, x + 1);
                return;
            }
        }
    }
}
