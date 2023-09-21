use proconio::input;

fn main() {
    input! {
        x: i64,
    };
    if 0 < x {
        println!("{}", (x / 10));
    } else {
        println!("{}", (x / 10) - if x % 10 != 0 { 1 } else { 0 });
    }
}
