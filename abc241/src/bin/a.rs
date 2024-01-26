use proconio::input;

fn main() {
    let n = 10;
    input! {
        a: [usize;n],
    };
    let mut k = 0;
    for _ in 0..3 {
        k = a[k];
    }
    println!("{k}");
}
