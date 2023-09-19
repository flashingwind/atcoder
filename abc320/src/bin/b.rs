use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut max_cnt = 0;
    for i in 0..s.len() {
        for j in (i + 1)..s.len() {
            let len = j - i;
            let mut is_ok = true;
            for k in 0..=len {
                let left = i + k; //→j
                let right = j - k; //→i
                if s[left] != s[right] {
                    is_ok = false;
                    break;
                }
            }
            if is_ok && max_cnt < len + 1 {
                max_cnt = len + 1;
            }
            // println!("max_cnt{max_cnt}");
        }
    }
    if max_cnt != 0 {
        println!("{}", max_cnt);
    } else {
        println!("1");
    }
}
