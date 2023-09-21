use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32;n],
        b: [i32;n],
    };
    let mut inner = 0;
    for i in 0..n {
        inner += a[i] * b[i];
    }
    if inner == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
