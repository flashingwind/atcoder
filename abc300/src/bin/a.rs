use proconio::input;

fn main() {
    input! {
        n: usize,
        a: u32,
        b: u32,
        c: [u32;n],
    };
    for i in 0..n {
        if c[i] == a + b {
            println!("{}", i + 1);
            return;
        }
    }
}
