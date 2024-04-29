use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [u32;n],
    };
    let mut cnt_ok = 0;
    for &ss in s.iter() {
        let mut is_ok = false;
        for a in 1..=1000 {
            for b in 1..=1000 {
                if ss == 4 * a * b + 3 * a + 3 * b {
                    is_ok = true;
                    cnt_ok += 1;
                    break;
                }
            }
            if is_ok {
                break;
            }
        }
    }
    println!("{}", n - cnt_ok);
}
