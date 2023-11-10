use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        k: usize,
    };
    for i in 0..k{
        if s[i]!='1'{
            println!("{}",s[i]);
            return;
        }
    }
    println!("1");
}
