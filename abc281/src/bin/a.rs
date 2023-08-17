use proconio::input;

fn main() {
    input! {
        n: u32,
    };
    for x in (0..=n).rev() {
        println!("{}", x);
    }
}
