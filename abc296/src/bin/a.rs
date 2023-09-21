use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: u32,
        s: Chars,
    };
    if n == 1 {
        println!("Yes");
        return;
    }
    let even = s[0];
    let odd = s[1];
    if even == odd {
        println!("No");
        return;
    }
    for (i, c) in s.iter().enumerate() {
        if i % 2 == 0 && even != *c || i % 2 == 1 && odd != *c {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
