use proconio::input;

fn main() {
    input! {
        k:u32,
        x:u32,
    };
    if x <= k * 500 {
        println!("Yes");
    } else {
        println!("No");
    }
}
