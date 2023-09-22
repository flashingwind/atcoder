use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        map: [u32;9],
    }
    let rows = [
        [0, 1, 2], // 上から 1 行目
        [3, 4, 5], // 上から 2 行目
        [6, 7, 8], // 上から 3 行目
        [0, 3, 6], // 左から 1 列目
        [1, 4, 7], // 左から 2 列目
        [2, 5, 8], // 左から 3 列目
        [0, 4, 8], // 左上から右下
        [2, 4, 6], // 右上から左下
    ];
    let mut g_cnt = 0u64;
    let mut cnt = 0u64;
    for order in (0..9).permutations(9) {
        let mut is_g = false;
        for &[i, j, k] in rows.iter() {
            if (map[i] == map[j] && order[i] < order[k] && order[j] < order[k])
                || (map[i] == map[k] && order[i] < order[j] && order[k] < order[j])
                || (map[j] == map[k] && order[j] < order[i] && order[k] < order[i])
            {
                is_g = true;
            }
        }
        if is_g {
            g_cnt += 1;
        }
        cnt += 1;
    }
    // println!("{}", 1. - (cnt / 362880.));
    // println!("{}", 1. - (cnt / (9. * 8. * 7.)));
    let ans = 1.0f64 - (g_cnt as f64 / cnt as f64);
    println!("{}", ans);
    // println!("g_cnt={g_cnt} cnt={cnt}");
}
