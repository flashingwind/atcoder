use proconio::{input, marker::Chars};

fn main() {
    input! {
        str: Chars,
    };
    let a = str[0].to_digit(10).unwrap() as i32;
    let b = str[1].to_digit(10).unwrap() as i32;
    let c = str[2].to_digit(10).unwrap() as i32;
    let d = str[3].to_digit(10).unwrap() as i32;
    let ops = ['+', '-'];
    for i in 0..2 {
        let mut sum = a;
        if i == 0 {
            sum += b;
        } else if i == 1 {
            sum -= b;
        }
        // println!("*{}{}{}={}", a, ops[i], b, sum);
        for j in 0..2 {
            let mut sum2 = sum;
            if j == 0 {
                sum2 += c;
            } else if j == 1 {
                sum2 -= c;
            }
            // println!("**{}{}{}{}{}={}", a, ops[i], b, ops[j], c, sum2);
            for k in 0..2 {
                let mut sum3 = sum2;
                if k == 0 {
                    sum3 += d;
                } else if k == 1 {
                    sum3 -= d;
                }
                // println!(
                //     "***{}{}{}{}{}{}{}={}",
                //     a, ops[i], b, ops[j], c, ops[k], d, sum3
                // );
                if sum3 == 7 {
                    println!("{}{}{}{}{}{}{}=7", a, ops[i], b, ops[j], c, ops[k], d);
                    return;
                }
            }
        }
    }
}
