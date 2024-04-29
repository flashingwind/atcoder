use proconio::{input, marker::Chars};

fn main() {
    let mut is_inside = false;
    input! {
        s: Chars,
    };
    for &c in s.iter() {
        if c == '|' && !is_inside {
            is_inside = true;
        } else if c == '|' && is_inside {
            is_inside = false;
        } else {
            if !is_inside {
                print!("{}", c);
            }
        }
    }
    println!();
}
