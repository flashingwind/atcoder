use proconio::input;

fn main() {
    input! {
        x: u64,
    };
    // b^p <= 10
    // p=1, b=10 x
    // p=2, b=3
    // b<=p√10
    let mut max = 1;
    for p in 2..f64::log(x as f64, 2.) as u32 {
        for b in 2..x / 2 + 1 {
            if x < b.pow(p) {
                break;
            }
            // print!("{}^{}=", b, p);
            // println!("{}", b.pow(p));
            if max < b.pow(p) {
                max = b.pow(p);
            }
        }
    }
    println!("{}", max);
}
