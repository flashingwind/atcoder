use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u32,
        a: [u32;n],
    };
    for i in 0..n {
        if a[i] == x {
            println!("{}", i + 1);
            return;
        }
    }
}
