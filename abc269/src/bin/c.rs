use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: u128,
    };
    let tmp = format!("{:b}", n);
    let bits = tmp.chars();
    let mut cnt_lines = 1;
    // println!("n={}", tmp);
    for b in bits.to_owned().sorted() {
        if b == '1' {
            cnt_lines *= 2;
        }
    }
    let mut x = vec![0u128; cnt_lines];
    let mut cnt = 0;
    for (_, b) in bits.into_iter().enumerate() {
        for (j, l) in x.iter_mut().enumerate() {
            *l *= 2;
            if b == '1' && (j + 1) / 2usize.pow(cnt) % 2 == 0 {
                *l += 1;
            }
        }
        if b == '1' {
            cnt += 1;
        }
    }
    for l in x.iter().sorted() {
        // println!("{:b}", l);
        println!("{}", l);
    }
}
