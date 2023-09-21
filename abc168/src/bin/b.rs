use proconio::{input, marker::Chars};

fn main() {
    input! {
        k: usize,
        s:Chars,
    };
    if s.len() <= k {
        println!("{}", s.iter().collect::<String>());
    } else {
        println!("{}...", s.get(0..k).unwrap().iter().collect::<String>());
    }
}
