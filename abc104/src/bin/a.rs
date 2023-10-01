use proconio::input;

fn main() {
    input! {
        r: u32,
    };
    if r < 1200 {
        println!("ABC");
    } else if r < 2800 {
        println!("ARC");
    } else {
        println!("AGC");
    }
}
