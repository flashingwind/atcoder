use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [[u32;w];h],
    };
    a.insert(0, vec![0; w + 1]);
    let pt = vec![vec![Vec::new(); w + 1]; h + 1];
}
fn dp(pt: Vec<Vec<u32>>, dir: char, x: usize, y: usize, h: usize, w: usize) {
    for i in x..=h {
        for j in y..=w {
            // from upper or from left
            if dir == 'd' {
            } else {
            }
        }
    }
}
