use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };
    let tmp = s.remove(0);
    s.push(tmp);
    println!("{}{}{}", s[0], s[1], s[2])
}
