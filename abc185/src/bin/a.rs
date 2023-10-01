use proconio::input;

fn main() {
    input! {
        a: [u32;4],
    };
    println!("{}", a.iter().min().unwrap());
}
