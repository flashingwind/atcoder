use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        mut t: Chars,
    };
    let mut cnt = 0;
    for i in 0..s.len() {
        if s[i] == t[i] {
            // println!("{i}: {}=={}", s[i], t[i]);
        } else if i <= s.len() - 2 && s[i] == t[i + 1] && s[i + 1] == t[i] {
            t[i] = s[i];
            t[i + 1] = s[i + 1];
            cnt += 1;
            // println!(
            //     "{i}: {}=={} {}=={} cnt={}",
            //     s[i],
            //     t[i],
            //     s[i + 1],
            //     t[i + 1],
            //     cnt
            // );
        } else {
            // println!("{i}: {}!={}", s[i], t[i]);
            println!("No");
            return;
        }
        if 1 < cnt {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
