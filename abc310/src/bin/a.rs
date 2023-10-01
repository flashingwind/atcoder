use proconio::input;

fn main() {
    input! {
        n: usize,
        p: u32,
        q: u32,
        mut d: [u32;n],
    };
    d.sort();
    if p < q + d[0] {
        println!("{}", p);
    } else {
        println!("{}", q + d[0]);
    }
}
