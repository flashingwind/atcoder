use proconio::input;

fn main() {
    input! {
        mut s: u32,
    };
    let mut m = s / 60;
    s %= 60;
    let h = m / 60;
    m %= 60;
    println!("{:<02}:{:<02}:{:<02}", h, m, s);
}
