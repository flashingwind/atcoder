use proconio::input;

fn main() {
    input! {
        mut n: u32,
    }
    let mut str = vec![];
    while n != 0 {
        str.push((n % 10).to_string());
        n /= 10;
    }
    for t in 0..4 - str.len() {
        str.push("0".to_string());
    }
    for ss in str.iter().rev() {
        print!("{}", ss);
    }
}
