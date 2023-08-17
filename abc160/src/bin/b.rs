use proconio::input;

fn main() {
    input! {
        mut x: u32,
    };
    let mut cnt = 0;

    cnt += x / 500 * 1000;
    x %= 500;
    cnt += x / 5 * 5;
    // x %= 5;
    println!("{}", cnt);
}
