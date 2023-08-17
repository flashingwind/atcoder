use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32;n],
    };
    for i in 0..n - 1 {
        if a[i] > a[i + 1] {
            println!("down {}", a[i] - a[i + 1]);
        } else if a[i] < a[i + 1] {
            println!("up {}", a[i + 1] - a[i]);
        } else {
            println!("stay");
        }
    }
}
