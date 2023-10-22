use proconio::input;

fn main() {
    input! {
        d: u64,
        t: u64,
        s: u64,
    };
    if d / s == t && d % s == 0 || d / s < t {
        println!("Yes");
    } else {
        println!("No");
    }
}
