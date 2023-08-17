use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        n: usize,
    };
    if x * 3 < y {
        println!("{}", x * n);
    } else {
        let m = n % 3;
        let l = (n - m) / 3;
        println!("{}", x * m + y * l);
    }
}
