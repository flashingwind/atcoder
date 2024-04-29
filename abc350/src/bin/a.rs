use proconio::input;
// use regex::Regex;

fn main() {
    input! {
        mut s: String,
    };
    s = s.split_off(3);
    let n = s.parse().unwrap();
    if n == 316 {
        println!("No");
    } else if 1 <= n && n <= 349 {
        println!("Yes");
    } else {
        println!("No");
    }
}
