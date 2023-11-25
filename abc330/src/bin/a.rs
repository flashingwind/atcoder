use proconio::input;

fn main() {
    input! {
        n: usize,
        l: u32,
        a: [u32;n],
    };
    let mut cnt = 0;
    for i in 0..n {
        if l <= a[i] {
            cnt += 1;
        }
    }
    println!("{cnt}");
}
