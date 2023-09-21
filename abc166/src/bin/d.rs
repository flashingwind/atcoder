use proconio::input;

#[inline(always)]
fn is_eq_b_pow_alpha(alpha: i128, b: i128) -> bool {
    let b_pow = b.pow(5);
    //println!("b={},b_pow={},alpha={}", b, b_pow, alpha);
    if -b_pow == alpha {
        return true;
    }
    false
}
fn is_divisible(
    x: i128,
    a_start: i128,
    a_end: i128,
    b_start: i128,
    b_end: i128,
) -> Option<(i128, i128)> {
    //println!("a={}~{} b={}~{}", a_start, a_end, b_start, b_end);
    for a_raw in a_start..=a_end {
        {
            let a = a_raw;
            let alpha = x - a.pow(5);
            for b_raw in b_start..=b_end {
                //println!("a_raw={} b_raw={}", a_raw, b_raw);
                if is_eq_b_pow_alpha(alpha, b_raw) {
                    return Some((a, b_raw));
                } else if is_eq_b_pow_alpha(alpha, -b_raw) {
                    return Some((a, -b_raw));
                }
            }
        }
        {
            let a = -a_raw;
            let alpha = x - a.pow(5);
            for b_raw in b_start..=b_end {
                //println!("a_raw={} b_raw={}", a_raw, b_raw);
                if is_eq_b_pow_alpha(alpha, b_raw) {
                    return Some((a, b_raw));
                } else if is_eq_b_pow_alpha(alpha, -b_raw) {
                    return Some((a, -b_raw));
                }
            }
        }
    }
    None
}

fn main() {
    input! {
        x: i128,
    };
    for d in (0..=10i128.pow(9)).step_by(10) {
        //for d in (0..=70).step_by(10) {
        //println!("d={}", d);
        if let Some((a_ans, b_ans)) = is_divisible(x, d, d + 10, d, d + 10) {
            println!("{} {}", a_ans, b_ans);
            return;
        } else if let Some((a_ans, b_ans)) = is_divisible(x, d, d + 10, 0, d) {
            println!("{} {}", a_ans, b_ans);
            return;
        } else if let Some((a_ans, b_ans)) = is_divisible(x, 0, d, d, d + 10) {
            println!("{} {}", a_ans, b_ans);
            return;
        }
    }
    println!("{} {}", 0, 0);
}
