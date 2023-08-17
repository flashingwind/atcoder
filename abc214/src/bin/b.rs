use proconio::input;

fn main() {
    input! {
        sum: u32,
        mult: u32,
    };
    let mut cnt = 0;
    for a in 0..=sum {
        for b in 0..=sum - a {
            for c in 0..=sum - a - b {
                if a + b + c <= sum && a * b * c <= mult {
                    cnt += 1;
                }
            }
        }
    }
    println!("{}", cnt);
}
