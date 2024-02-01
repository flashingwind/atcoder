use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let mut si = vec![];
    let mut ti = vec![];
    let dict: Vec<char> = ('a'..='z').collect();
    for i in 0..s.len() {
        si.push(dict.iter().position(|c| *c == s[i]).unwrap() as i32);
        ti.push(dict.iter().position(|c| *c == t[i]).unwrap() as i32);
    }
    let d = ti[0] - si[0];

    for i in 0..s.len() {
        if si[i] < 0 {
            si[i] += dict.len() as i32;
        }
        si[i] = (si[i] + d) % dict.len() as i32;
        // println!("{}=={}", si[i], ti[i]);
        if si[i] != ti[i] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
