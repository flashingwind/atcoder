use proconio::{input, marker::Chars};

fn main() {
    input! {
        c: Chars,
    };
    if c[0] == c[1] && c[1] == c[2] {
        println!("Won");
    } else {
        println!("Lost");
    }
}
