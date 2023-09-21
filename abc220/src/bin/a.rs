use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
    };
    let m = a % c;
    let mul = (a / c + if m == 0 { 0 } else { 1 }) * c;
    if a <= mul && mul <= b {
        println!("{}", mul);
    } else {
        println!("-1");
    }
}
