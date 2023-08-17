use proconio::input;

fn main() {
    input! {
        mut n: u32,
        k: u32,
        x: u32,
        y: u32,
    };
    let mut cost = 0;
    if n <= k {
        cost += n * x;
    } else {
        cost += k * x;
        n -= k;
        cost += n * y;
    }
    println!("{}", cost);
}
