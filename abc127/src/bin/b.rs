use proconio::input;

fn main() {
    input! {
        r: i128,
        d: i128,
        mut x: i128,
    };
    for _y in 1..=10 {
        x = r * x - d;
        println!("{}", x);
    }
}
