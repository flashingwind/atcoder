use proconio::input;

fn main() {
    input! {
        w1: u32,
        c1: u32,
        w2: u32,
    };
    if w2 > (w1 - c1) {
        println!("{}", w2 - (w1 - c1));
    } else {
        println!("0",);
    }
}
