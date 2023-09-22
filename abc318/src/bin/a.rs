use proconio::input;

fn main() {
    input! {
        mut n: u64,
        m: u64,
        p: u64,
    }
    let mut cnt = 0;
    if m <= n {
        n -= m;
        cnt += 1;
        if p <= n {
            cnt += (n / p);
            // n /= p;
        }
    }
    println!("{}", cnt);
}
