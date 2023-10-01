use proconio::input;

fn main() {
    input! {
        k: u32,
        a: u32,
        b: u32,
    };
    if a % k == 0 || b % k == 0 {
        println!("OK");
        return;
    }
    if (a / k) * k + k <= b {
        println!("OK");
    } else {
        println!("NG");
    }
}
