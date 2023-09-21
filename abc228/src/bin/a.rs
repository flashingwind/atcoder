use proconio::input;

fn main() {
    input! {
        s: usize,
        mut t: usize,
        mut x: usize,
    };
    if t < s {
        t += 24;
    }
    if s <= x && x < t || x < s && x + 24 < t {
        println!("Yes");
    } else {
        println!("No");
    }
}
