use proconio::input;

fn main() {
    input! {
        n: usize,
        th: u32,
        h: [u32;n],
    };
    let cnt = h.iter().filter(|v| th <= **v).count();
    println!("{}", cnt);
}
