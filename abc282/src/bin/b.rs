use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        _m: usize,
        ss: [Chars;n],
    };
    let mut cnt = 0;
    for (k, p1) in ss.iter().enumerate() {
        for (l, p2) in ss.iter().skip(k).enumerate() {
            if k == k + l {
                continue;
            }
            let mut is_ok = true;
            for (i, _) in p1.iter().enumerate() {
                if p1[i] == 'x' && p2[i] == 'x' {
                    // println!("x: {}!=x && {}!=x", p1[i], p2[i]);
                    is_ok = false;
                    break;
                }
            }
            if is_ok {
                cnt += 1;
                // println!("{k}{:?},\n{}{:?}", *p1, k + l, *p2);
            }
        }
    }
    println!("{}", cnt);
}
