use proconio::input;

fn main() {
    input! {
        n: usize,
        mut h: [u32;n],
    };
    h.sort();
    h.dedup();
    println!("{}", h.len());
}
