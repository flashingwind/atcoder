use proconio::input;

fn main() {
    input! {
        x: i32,
        a: i32,
        b: i32,
    };

    if b - a <= 0 {
        println!("delicious");
    } else if b - a <= x {
        println!("safe");
    } else {
        println!("dangerous");
    }
}
