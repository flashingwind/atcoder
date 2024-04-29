use proconio::input;

fn main() {
    input! {
        v: u32,
        s: u32,
        t: u32,
        d: u32,
    };
    // println!("{} <= {}, {} <= {}", s * v, d, d, t * v);
    if s * v <= d && d <= t * v {
        println!("No");
    } else {
        println!("Yes");
    }
}
