use proconio::input;

fn main() {
    input! {
        mut a: u32,
        mut b: u32,
    };
    if a == 1 {
        a = 14;
    }
    if b == 1 {
        b = 14;
    }
    if b < a {
        println!("Alice");
    } else if a < b {
        println!("Bob");
    } else {
        println!("Draw");
    }
}
