use proconio::{input, marker::Chars};
use rustc_hash::FxHashMap;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut map: [Chars;h],
    }
    let mut clr_h = vec![&mut FxHashMap::default(); h];
    let mut clr_w = vec![&mut FxHashMap::default(); w];
    let mut is_changed;
    for _ in 0..100 {
        is_changed = false;
        let mut to_be_clear_row = Vec::new();
        for i in 0..h {
            let mut clr: FxHashMap<_, _> = clr_h[i].to_owned();
            for j in 0..w {
                if map[i][j] != '.' {
                    *clr.entry(map[i][j]).or_insert(0) + 1;
                }
            }
            if clr.len() == 1 && *clr.get(clr.keys().next().unwrap()).unwrap() <= 2 {
                to_be_clear_row.push(i);
                is_changed = true;
            }
            clr_h[i] = &mut clr;
        }
        let mut to_be_clear_col = Vec::new();
        for j in 0..w {
            let mut clr: FxHashMap<_, _> = clr_w[j].to_owned();
            for i in 0..h {
                if map[i][j] != '.' {
                    *clr.entry(map[i][j]).or_insert(0) + 1;
                }
            }
            if clr.len() == 1 && *clr.get(clr.keys().next().unwrap()).unwrap() <= 2 {
                to_be_clear_col.push(j);
                is_changed = true;
            }
            clr_w[j] = &mut clr;
        }
        for &j in to_be_clear_col.iter() {
            for i in 0..h {
                map[i][j] = '.';
            }
        }
        for &i in to_be_clear_row.iter() {
            for j in 0..w {
                map[i][j] = '.';
            }
        }
        // println!("{:?}", map);
        if !is_changed {
            break;
        }
    }
    let mut cnt = 0;
    for i in 0..h {
        for j in 0..w {
            if map[i][j] != '.' {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}
