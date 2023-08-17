use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars;2],
    };
    let mut cnt = Vec::new();
    cnt.push(s[0].iter().filter(|a| **a == '#').count());
    cnt.push(s[1].iter().filter(|a| **a == '#').count());
    if cnt[0] == 1 && cnt[1] == 1 {
        if s[0][0] == '#' && s[1][1] == '#' || s[0][1] == '#' && s[1][0] == '#' {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
