use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    let mut d: HashMap<char, (i64, i64)> = HashMap::new();

    d.insert('L', (0, -1));
    d.insert('R', (0, 1));
    d.insert('U', (-1, 0));
    d.insert('D', (1, 0));

    input! {
        h: usize,
        w: usize,
        _n: usize,
        t: Chars,
        map: [Chars;h],
    };
    let mut cnt = 0;

    for ii in 0..w {
        for jj in 0..h {
            let mut i = ii;
            let mut j = jj;
            let mut is_ok = true;
            if h <= j || w <= i {
                break;
            }
            // println!("{},{}: _,_ = {}", i + 1, j + 1, map[i][j]);
            for tt in t.iter() {
                let didj = d.entry(*tt).or_default();
                let di = didj.0;
                let dj = didj.1;
                if (i as i64) < di
                    || (j as i64) < dj
                    || (h as i64) <= (j as i64 + dj)
                    || (w as i64) <= (i as i64 + di)
                {
                    break;
                }
                // println!(
                //     "{},{}: {},{} = {} {}:{},{} {}",
                //     ii + 1,
                //     jj + 1,
                //     i + 1,
                //     j + 1,
                //     map[i][j],
                //     tt,
                //     di,
                //     dj,
                //     map[i][j]
                // );
                if map[i][j] == '#' {
                    is_ok = false;
                    break;
                }
                i = ((i as i64) + di) as usize;
                j = ((j as i64) + di) as usize;
                if w < i || h < j {
                    break;
                }
            }
            if is_ok {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}
