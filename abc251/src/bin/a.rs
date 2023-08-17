use proconio::input;

fn main() {
    input! {
        s: String,
    };
    for _ in 1..=(6 / s.len()) {
        print!("{}", s);
    }
    println!();
}
