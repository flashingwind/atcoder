use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
    };
    {
        let mut i = 0usize;
        let mut cnt = 0usize;
        while i < n + cnt - 1 {
            // println!("n={} i={},i+1={}", n, i, i + 1);
            if s[i] == 'n' && s[i + 1] == 'a' {
                s.insert(i + 1, 'y');
                i += 3;
                cnt += 1;
            } else {
                i += 1;
            }
        }
    }
    for c in s.iter() {
        print!("{}", *c);
    }
    println!();
}
