use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [i32;n],
    };
    let mut max = 0;
    for p_i in p.iter() {
        if max < *p_i {
            max = *p_i;
        }
    }
    let mut sum = 0;
    let mut is_coupon_used = false;
    for p_i in p.iter() {
        if !is_coupon_used && max == *p_i {
            sum += *p_i / 2;
            is_coupon_used = true;
        } else {
            sum += *p_i;
        }
    }
    println!("{}", sum);
}
