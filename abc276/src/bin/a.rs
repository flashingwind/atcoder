use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut last_a = usize::MAX;
    for (i, c) in s.iter().enumerate() {
        if *c == 'a' {
            last_a = i;
        }
    }
    if last_a == usize::MAX {
        println!("-1");
    } else {
        println!("{}", last_a + 1);
    }
}
