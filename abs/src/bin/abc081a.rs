use proconio::input;

fn main() {
    input! {
        cs: String,   // read as Vec<char>
    }
    let mut sum = 0;
    for c in cs.chars() {
        sum += c as i32 - 48;
    }
    println!("{}", sum);
}
