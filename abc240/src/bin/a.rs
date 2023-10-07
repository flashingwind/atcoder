use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    if a.abs_diff(b) == 1 || (a == 1 && b == 10) || (b == 1 && a == 10) {
        println!("Yes");
    } else {
        println!("No");
    }
}
