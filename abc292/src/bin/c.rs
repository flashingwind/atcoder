use proconio::input;

fn main() {
    input! {
        n: u64,
    };
    let mut ans = 0;
    for i in 1..n {
        let mut xx: u64 = 0;
        {
            let x = i;
            let mut j = 1;
            while j * j <= x {
                if x % j == 0 {
                    xx += 1;
                    if x != j * j {
                        xx += 1;
                    }
                }
                j += 1;
            }
        }
        let mut yy: u64 = 0;
        {
            let y = n - i;
            let mut j = 1;
            while j * j <= y {
                if y % j == 0 {
                    yy += 1;
                    if y != j * j {
                        yy += 1;
                    }
                }
                j += 1;
            }
        }
        ans += xx * yy;
    }
    println!("{}", ans);
}
