use proconio::input;

fn main() {
    input! {
        M: u32,
        D: u32,
        mut y: u32,
        mut m: u32,
        mut d: u32,
    };
    y -= 1;
    m -= 1;
    // d += 1;
    m += d / D;
    d %= D;
    y += m / M;
    m %= M;
    println!("{} {} {}", y + 1, m + 1, d + 1);
}
