use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
    };
    let mut sum = 0;
    if a < 7 {
        sum += 7 - a;
    } else {
        sum += 1;
    }
    if b < 7 {
        sum += 7 - b;
    } else {
        sum += 1;
    }
    if c < 7 {
        sum += 7 - c;
    } else {
        sum += 1;
    }
    println!("{}", sum);
}
