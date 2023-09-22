use proconio::input;

fn main() {
    input! {
        mut x: u64,
        k: u32,
    }
    for i in 0..k {
        let d = 10u64.pow(i + 1);
        let m = x % d;
        // println!("{x} m={m} d={d}");
        x = (x / d) * d;
        // println!("x={x}");

        if 5 * d / 10 <= m {
            x += d;
        }
        // println!("x={x}");
    }
    println!("{}", x);
}
