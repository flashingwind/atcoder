use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars,
    };
    for c in x.iter() {
        if *c == '.' {
            break;
        } else {
            print!("{}", c);
        }
    }
    println!();
}
