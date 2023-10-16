use proconio::input;

fn main() {
    input! {
        r: u32,
        g: u32,
        b: u32,
    };
    if (100 * r + 10 * g + b) % 4 == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}
