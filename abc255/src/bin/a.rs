use proconio::{input, marker::Usize1};

fn main() {
    input! {
        r: Usize1,
        c: Usize1,
        a: [[u32;2];2],
    };
    println!("{}", a[r][c])
}
