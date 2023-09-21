use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
        //d: [i32;n],
    };
    for _ in 0..t {
        input! {
            n: usize,
            mut s: Chars,
        };
        let mut cnt = 0;
        for i in (0..n).step_by(2) {
            for j in (i + 2..n).rev() {
                if i % 2 == 0 && j % 2 == 0 && 2 + i <= j && s[i] == '1' && s[j] == '1' {
                    // println!("{:?}", s);
                    s[i] = '0';
                    s[j] = '0';
                    cnt += 1;
                    // println!("{:?}", s);
                    break;
                }
            }
        }
        for i in (0..n).step_by(2) {
            for j in (i + 2..n).rev() {
                if i % 2 == 1 && j % 2 == 1 && 2 + i <= j && s[i] == '1' && s[j] == '1' {
                    // println!("{:?}", s);
                    s[i] = '0';
                    s[j] = '0';
                    cnt += 1;
                    // println!("{:?}", s);
                    break;
                }
            }
        }
        let mut sum_odd = 0;
        let mut sum_eve = 0;
        for (i, c) in s.iter().enumerate() {
            if *c == '1' {
                if i % 2 == 0 {
                    sum_eve += 1;
                } else {
                    sum_odd += 1;
                }
            }
        }
        // println!("sum_eve = {} || sum_odd = {}", sum_eve, sum_odd);
        if sum_eve % 2 != 0 || sum_odd % 2 != 0 {
            println!("-1");
        } else {
            cnt += sum_eve / 2 + sum_odd / 2;
            println!("{}", cnt);
        }
        // let mut is_ok = true;
        // for c in s.iter() {
        //     if *c == '1' {
        //         is_ok = false;
        //         // println!("-1");
        //         break;
        //     }
        // }
        // if is_ok {
        // // println!("{}", cnt);
        // }
    }
}
