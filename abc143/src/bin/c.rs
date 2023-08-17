use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: Chars,
    };
    let mut cnt = 1usize;
    for i in 0..n - 1 {
        if a[i] != a[i + 1] {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
