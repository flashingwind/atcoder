use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
    };
    if 0 < a && b == 0 {
        println!("Gold");
    } else if a == 0 && 0 < b {
        println!("Silver");
    } else if 0 < a && 0 < b {
        println!("Alloy");
    }
}
