use proconio::input;

fn main() {
    input! {
        r: u32,
    };
    println!("{}", 99-r%100+1);
}