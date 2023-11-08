use proconio::input;

fn main() {
    input! {
        xx: i64,
        mut k: i64,
        d: i64,
    };
    let mut x = xx.abs();
    let t = x / d;
    // println!("t={t}");
    if t <= k {
        k -= t; // 回数
        x -= t * d; // 距離
    } else {
        x -= k * d;
        // k = 0;
        println!("{}", x.abs());
        return;
    }
    // println!("k={k} x={x}");
    k %= 2;
    if k == 1 {
        // k2 = 0;
        x -= d;
    }
    println!("{}", x.abs());
}
