use proconio::input;

fn main() {
    input! {
        h: u32,
        a: u32,
    };
    println!("{}", h / a + if h % a == 0 { 0 } else { 1 });
}
