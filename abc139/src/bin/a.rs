use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        forecasts: Chars,
        reals: Chars,
    };
    let mut cnt = 0;
    for (i, f) in forecasts.iter().enumerate() {
        if *f == reals[i] {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
