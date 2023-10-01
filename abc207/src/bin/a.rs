use proconio::input;

fn main() {
    input! {
        n: [u32;3],
    };
    let min = n.iter().min().unwrap();
    println!("{}", n[0] + n[1] + n[2] - min);
}
