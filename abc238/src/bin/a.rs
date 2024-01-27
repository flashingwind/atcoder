use proconio::input;

fn main() {
    input! {
        n: u32,
    };
    if n < 2 {
        println!("Yes");
    } else if n <= 4 {
        println!("No");
    } else {
        println!("Yes");
    }
}
