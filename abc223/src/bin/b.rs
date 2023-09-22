use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut ss = Vec::new();
    for i in 0..s.len() {
        let mut e = Vec::new();
        for j_raw in i..i + s.len() {
            let j = j_raw % s.len();
            e.push(s[j]);
        }
        ss.push(e);
    }
    ss.sort();
    println!(
        "{}",
        ss[0].iter().map(|v| v.to_string()).collect::<String>()
    );
    println!(
        "{}",
        ss.last()
            .unwrap()
            .iter()
            .map(|v| v.to_string())
            .collect::<String>()
    );
}
