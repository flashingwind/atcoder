use proconio::input;

fn main() {
    input! {
        k: u32,
    };
    let h = 21 + (k / 60);
    let m = k % 60;
    println!("{h}:{m:0>2}");
}
