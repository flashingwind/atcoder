use proconio::{input, marker::Chars};
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    input! {
        num_mv: usize,//N回の移動をします
        num_redb: usize,//M 個の体力を回復するアイテム
        mut hp: usize,//高橋君の体力は H
        k: usize,
        s: Chars,
        redb_xy: [(i64,i64);num_redb],
    };
    let mut redb: HashSet<(i64, i64)> = HashSet::from_iter(redb_xy.iter().cloned());
    // 高橋君の体力が負になった場合、高橋君は倒れてしまい、移動をやめる。
    // そうでない場合、移動した点にアイテムがあり、
    // かつ高橋君の体力が K 未満ならば高橋君の体力が K になる。
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    for t in 0..num_mv {
        if s[t] == 'R' {
            x += 1;
        } else if s[t] == 'L' {
            x -= 1;
        } else if s[t] == 'U' {
            y += 1;
        } else if s[t] == 'D' {
            y -= 1;
        }
        if 0 == hp {
            println!("No");
            return;
        } else {
            hp -= 1;
        }
        if hp < k {
            if redb.contains(&(x, y)) {
                hp = k;
                redb.remove(&(x, y));
            }
        }
    }
    println!("Yes");
}
