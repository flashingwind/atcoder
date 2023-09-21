use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    };
    let k = (b + a) / 2;
    if 2 * k == (b + a) {
        println!("{}", k);
    } else {
        println!("IMPOSSIBLE");
    }
}
