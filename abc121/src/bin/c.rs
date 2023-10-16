use proconio::input;

fn main() {
    input! {
        n: usize,
        mut m: u64,
        mut ab: [(u64,u64);n],
    };
    ab.sort();
    let mut cost = 0u64;
    for &(p, b) in ab.iter() {
        if b <= m {
            cost += b * p;
            m -= b;
        } else {
            cost += m * p;
            m = 0;
        }
    }
    println!("{}", cost);
}
