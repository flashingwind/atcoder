use proconio::input;

fn main() {
    input! {
        n: u32,
        a: u32,
        x: u32,
        y: u32,
    }
    let mut cst = 0;
    if n < a {
        cst += n * x;
    } else {
        cst += a * x;
        cst += (n - a) * y;
    }
    println!("{}", cst);
}
