use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32;n],
    };
    a.sort();
    println!("{}", a.last().unwrap() - a.first().unwrap());
}
