use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let cs = "0123456789".chars();
    for c in cs {
        if !s.contains(&c) {
            println!("{c}");
        }
    }
}
