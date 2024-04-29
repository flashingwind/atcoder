use proconio::input;

fn main() {
    input! {
        n: f32,
    };
    let a = n.floor();
    let b = (n - a) * 10.0;
    if b < 5.0 {
        println!("{}", a as u32)
    } else {
        println!("{}", a as u32 + 1)
    }
}
