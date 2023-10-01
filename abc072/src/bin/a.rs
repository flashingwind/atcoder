use proconio::input;

fn main() {
    input! {
        x: u32,
        t: u32,
    };
    println!("{}", if t <= x { x - t } else { 0 });
}
