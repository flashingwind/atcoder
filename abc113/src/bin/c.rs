use std::vec;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let mut city = Vec::new();
    let mut prov_cnt = vec![0; n + 1];
    for _ in 0..m {
        input! {
            pid: usize,
            y: u64,
        };
        city.push((pid, y, 0usize));
    }

    for (pid, _, cid) in city
        .iter_mut()
        .sorted_by(|(_, ya, _), (_, yb, _)| ya.cmp(yb))
    {
        prov_cnt[*pid] += 1;
        *cid = prov_cnt[*pid];
    }
    for (pid, _, cnt) in city.iter() {
        println!("{:>06}{:>06}", pid, cnt);
    }
}
