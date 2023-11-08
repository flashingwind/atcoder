use proconio::input;

fn main() {
    input! {
        x: u64,
        y: u64
    };
    let mut c = 1;
    let mut cnt = 0;
    while x * c <= y {
        c *= 2;
        cnt += 1;
    }
    println!("{}", cnt);
}
