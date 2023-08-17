use proconio::input;

fn main() {
    input! {
        mut n: [u32;6],
    };
    n.sort();
    println!("{}", n[3]);
}
