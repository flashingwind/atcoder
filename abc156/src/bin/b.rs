use proconio::input;

fn main() {
    input! {
        mut n: u64,//<=10^9
        k: u64, // k-base, 2<=k<=10
    };
    let mut cnt = 0;
    while 0 < n {
        n /= k;
        cnt += 1;
    }
    println!("{}", cnt);
}
