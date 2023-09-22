use proconio::input;

fn main() {
    input! {
        n: usize,
        mut b: [u32;n],
    }
    b.sort();
    for i in 1..n {
        if b[i - 1] + 1 != b[i] {
            println!("{}", b[i - 1] + 1);
        }
    }
}
