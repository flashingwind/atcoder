use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        mut s: Chars,
    };
    let mut is_inside = false;
    for c in s.iter_mut() {
        if *c == '"' {
            if is_inside {
                is_inside = false;
            } else {
                is_inside = true;
            }
        } else if !is_inside && *c == ',' {
            *c = '.';
        }
        print!("{}", *c);
    }
    println!();
}
