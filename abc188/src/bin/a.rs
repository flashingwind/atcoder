use proconio::input;

fn main() {
    input! {
        x: i16,
        y: i16,
    };
    if (x - y).abs() < 3 {
        println!("Yes");
    } else {
        println!("No")
    }
}
