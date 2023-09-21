use proconio::input;

fn main() {
    input! {
        n :u64,
    };
    let mut cnt: u64 = 0;
    for a in 1..n {
        for b in 1..=(n - 1) / a {
            if a * b < n {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}
