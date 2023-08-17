use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut cnt = 0;
    for c in s.iter() {
        if *c == 'v' {
            cnt += 1;
        } else if *c == 'w' {
            cnt += 2;
        }
    }
    println!("{}", cnt);
}
