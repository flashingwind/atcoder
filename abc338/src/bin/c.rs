use proconio::input;

fn main() {
    input! {
        n: usize,
        q: [u64;n],
        a: [u64;n],
        b: [u64;n],
    };
    let mut max_a_cnt = u64::MAX;
    for i in 0..n {
        if a[i] != 0 {
            max_a_cnt = max_a_cnt.min(q[i] / a[i]);
        }
    }
    // println!("max_a_cnt={max_a_cnt}");
    // println!("max_b_cnt={max_b_cnt}");
    let mut max_cnt = 0;
    for cnt_a in 0..=max_a_cnt {
        let mut qq = q.clone();
        for i in 0..n {
            qq[i] -= a[i] * cnt_a;
        }
        // println!("cnt_a={cnt_a}");
        let mut cnt_b = u64::MAX;
        for i in 0..n {
            if b[i] != 0 {
                if qq[i] < b[i] {
                    break;
                } else {
                    cnt_b = cnt_b.min(qq[i] / b[i]);
                }
            }
        }
        if cnt_b == u64::MAX {
            break;
        }
        max_cnt = max_cnt.max(cnt_a + cnt_b);
    }
    // println!("  cnt_b={cnt_b} cnt={cnt}");
    // println!("  {:?}", qq);

    println!("{}", max_cnt);
}
