use proconio::input;

fn main() {
    input! {
        mut a: u32,
        mut b: u32,
    };
    let mut sum = 0;
    if a < b {
        sum += b;
        b -= 1;
    } else {
        sum += a;
        a -= 1;
    }
    if a < b {
        sum += b;
    } else {
        sum += a;
    }
    println!("{}", sum);
}
