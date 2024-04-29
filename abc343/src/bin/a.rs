use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    };
    for i in 0..=9 {
        if i != a + b {
            println!("{i}");
            return;
        }
    }
}
