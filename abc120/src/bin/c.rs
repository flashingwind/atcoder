use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    // println!("{:?}", s);
    let mut cnt0 = s.iter().filter(|&&c| c == '0').count();
    let mut cnt1 = s.iter().filter(|&&c| c == '1').count();
    println!("{}", cnt0.min(cnt1) * 2);
}
