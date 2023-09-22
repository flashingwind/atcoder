use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    for t in (0..n).step_by(2) {
        if s[t] == '1' {
            println!("Takahashi");
            return;
        } else if s[t + 1] == '1' {
            println!("Aoki");
            return;
        }
    }
}
