use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [u32;n],
    };
    let max = h.iter().max().unwrap();
    println!("{}", h.iter().position(|v| *v == *max).unwrap() + 1);
}
