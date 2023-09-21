use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        k: usize,
        s: Chars,
    };
    let mut cnt = 0;
    for c in s.iter() {
        if *c == 'o' {
            cnt += 1;
        }
        if k < cnt {
            print!("x");
        } else {
            print!("{}", c);
        }
    }
    println!();
}
