use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut cnt = 0;
    let mut max = 0;
    for c in s.iter() {
        if *c == 'R' {
            cnt += 1;
            max = max.max(cnt);
        } else {
            cnt = 0;
        }
    }
    println!("{}", max);
}
