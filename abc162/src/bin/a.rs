use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    };
    for d in n.iter() {
        if *d == '7' {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
