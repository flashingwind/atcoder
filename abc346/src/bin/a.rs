use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32;n],
    };
    let mut is_first = true;
    for i in 0..n - 1 {
        if !is_first {
            print!(" {}", a[i] * a[i + 1]);
        } else {
            is_first = false;
            print!("{}", a[i] * a[i + 1]);
        }
    }
    println!();
}
