use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    };
    let v = vec![1_u32, 2, 3];
    println!(
        "{}",
        v.iter().filter(|n| **n != a && **n != b).next().unwrap()
    );
}
