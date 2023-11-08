use proconio::input;

fn main() {
    input! {
        n: usize,
        mut m: [(String,u32);n],
    };
    m.sort_by(|a, b| b.1.cmp(&a.1));
    println!("{}", m[1].0)
}
