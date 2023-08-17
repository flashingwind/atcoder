use proconio::input;

fn main() {
    input! {
        s: Chars,
    };
    let mut sum = 0;
    for c in s.iter() {
        sum += c;
    }
}
