use proconio::input;

fn main() {
    input! {
        c: char,
    };
    println!("{}", (c as u8 + 1u8) as char);
}
