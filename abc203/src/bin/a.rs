use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
    };
    if a == b {
        println!("{}", c);
    } else if b == c {
        println!("{}", a);
    } else if a == c {
        println!("{}", b);
    } else {
        println!("0");
    }
}
