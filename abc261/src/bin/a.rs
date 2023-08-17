use proconio::input;

fn main() {
    input! {
        l1: usize,
        r1: usize,
        l2: usize,
        r2: usize,
    };
    let left = l1.max(l2);
    let right = r1.min(r2);
    if left < right {
        println!("{}", right - left);
    } else {
        println!("0");
    }
}
