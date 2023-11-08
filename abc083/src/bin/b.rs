use proconio::input;

fn main() {
    input! {
        n: u32,
        a: u32,
        b: u32,
    };
    let mut ans = 0;
    for t in 1..=n {
        let mut sum = 0;
        for c in t.to_string().chars() {
            sum += c.to_digit(10).unwrap();
        }
        if a <= sum && sum <= b {
            ans += t;
        }
    }
    println!("{}", ans);
}
