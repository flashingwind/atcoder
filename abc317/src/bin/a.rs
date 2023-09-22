use proconio::input;

fn main() {
    input! {
        n: usize,
        hp: u32,
        x: u32,
        p: [u32;n],
    }
    for i in 0..n {
        if x <= hp + p[i] {
            println!("{}", i + 1);
            break;
        }
    }
}
