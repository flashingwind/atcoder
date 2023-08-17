use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
    };
    for i in (0..n * 2).step_by(2) {
        s.insert(i + 1, s[i]);
    }
    print!("{}", s.iter().collect::<String>());
}
