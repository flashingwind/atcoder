use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let mut is_ok = false;
    for c in s.iter() {
        if *c == 'C' {
            is_ok = true;
        }
        if is_ok && *c == 'F' {
            print!("Yes");
            return;
        }
    }
    println!("No");
}
