use proconio::input;

fn main() {
    input! {
        n: usize,
        a: u32,
        b: u32,
        p: [u32;n],
    };
    let min = p
        .iter()
        .filter(|v| **v <= a)
        .count()
        .min(p.iter().filter(|v| a < **v && **v <= b).count())
        .min(p.iter().filter(|v| b < **v).count());
    println!("{}", min);
}
