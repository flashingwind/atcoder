use proconio::input;

fn main() {
    input! {
        r: u64,
        g: u64,
        b: u64,
        c: String,
    };
    if c == "Red" {
        println!("{}", if g < b { g } else { b });
    } else if c == "Green" {
        println!("{}", if r < b { r } else { b });
    } else if c == "Blue" {
        println!("{}", if g < r { g } else { r });
    }
}
