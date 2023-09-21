use proconio::input;

fn main() {
    input! {
        mut a: u64,
        mut b: u64,
    };
    let mut cnt = 0;
    loop {
        if b < a {
            if b == 0 {
                cnt -= 1;
                break;
            } else if a % b == 0 {
                cnt += a / b - 1;
                a = a % b + b;
            } else {
                cnt += a / b;
                a = a % b;
            }
        } else if a < b {
            if a == 0 {
                cnt -= 1;
                break;
            } else if b % a == 0 {
                cnt += b / a - 1;
                b = b % a + a;
            } else {
                cnt += b / a;
                b = b % a;
            }
        } else {
            break;
        }
        // println!("{},{}", a, b);
    }
    println!("{}", cnt);
}
