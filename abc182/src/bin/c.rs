use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    };
    let m = n.len();
    let mut max_sum_cnt = 0;
    let mut is_ok = false;
    for bits in 0usize..(1 << m) {
        let mut sum = 0;
        let mut sum_cnt = 0;
        for d in 0..m {
            if (bits >> d & 1) == 1 {
                sum += n[d].to_digit(10).unwrap();
                sum_cnt += 1;
            }
        }
        // println!("sum={sum} sum");
        if sum % 3 == 0 && max_sum_cnt < sum_cnt {
            is_ok = true;
            max_sum_cnt = sum_cnt;
        }
    }
    if is_ok {
        println!("{}", m - max_sum_cnt);
    } else {
        println!("-1");
    }
}
