use proconio::input;

fn main() {
    input! {
        a: u128,
        b: u128,
        x: u128,
    };
    let fa;
    if a != 0 {
        fa = (a - 1) / x + 1;
    } else {
        fa = 0;
    }
    let fb;
    if b != 0 {
        fb = b / x + 1;
    } else {
        fb = 0;
    }
    println!("{}", fb - fa);
}
