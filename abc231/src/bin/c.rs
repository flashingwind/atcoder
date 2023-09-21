use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize;n],
        ths: [usize;q],
    };
    a.sort();
    a.reverse();
    let mut cnt = vec![0; q];
    let mut cnt_before = 0;
    for (i, th) in ths.iter().enumerate().sorted_by(|a, b| a.1.cmp(&b.1)).rev() {
        cnt[i] = cnt_before;
        for (_j, a) in a.iter().enumerate().skip(cnt[i]) {
            if *th <= *a {
                cnt[i] += 1;
            } else {
                break;
            }
        }
        cnt_before = cnt[i];
    }

    for c in cnt.iter() {
        println!("{}", *c);
    }
}
