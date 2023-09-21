use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [u32;n],
    };
    let mut cnt = 0;
    let mut max_cnt = 0;
    let mut h_left = h[0];
    let mut itr_h = h.iter();
    itr_h.next();
    for (_, h_i) in itr_h.enumerate() {
        if h_left >= *h_i {
            cnt += 1;
            if max_cnt < cnt {
                max_cnt = cnt;
            }
        } else {
            // 高いのが来ちゃったので、リセット
            cnt = 0;
        }
        h_left = *h_i;
    }
    println!("{}", max_cnt);
}
