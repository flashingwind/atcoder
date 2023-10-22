use proconio::input;

fn main() {
    input! {
        n: u64,
    };
    let mut b = 1;
    let mut min = u64::MAX;
    for b_cnt in 0..=60 {
        let a = n / b;
        let c = n - a * b;
        let sum = a + b_cnt + c;
        if sum < min {
            // println!("{a}*2^{b_cnt}+{c}");
            min = sum;
        }
        b *= 2;
    }
    println!("{}", min);
}
