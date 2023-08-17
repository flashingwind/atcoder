use proconio::input;

fn main() {
    input! {
        mut n: u128,
    };
    let mut ans: u128 = 0;
    for t in 1usize..=5 {
        if n >= 10u128.pow(3 * t as u32) {
            ans += n - "9".repeat(3 * t).parse::<u128>().unwrap();
        }
    }
    println!("{}", ans);
}
