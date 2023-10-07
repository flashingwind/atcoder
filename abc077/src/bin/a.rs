use proconio::{input, marker::Chars};

fn main() {
    input! {
        c: [Chars;2],
    };
    for i in 0..3 {
        if c[0][i] != c[1][2 - i] {
            println!("NO");
            return;
        }
    }
    println!("YES");
}
