use proconio::input;

fn main() {
    input! {
        k: u8,
    };
    for a in b'A'..=(b'A' + k - 1) {
        print!("{}", a as char);
    }
    println!();
}
