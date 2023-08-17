use proconio::input;

fn main() {
    input! {
        m: u64,
        h: u64,
    };
    if h % m == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
