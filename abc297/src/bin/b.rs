use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut b1 = std::usize::MAX;
    let mut b2 = std::usize::MAX;
    let mut r1 = std::usize::MAX;
    let mut r2 = std::usize::MAX;
    let mut k = std::usize::MAX;
    for i in 0..8 {
        if s[i] == 'B' {
            if b1 == std::usize::MAX {
                b1 = i;
            } else {
                b2 = i;
                if b1 % 2 == b2 % 2 {
                    println!("No");
                    return;
                }
            }
        } else if s[i] == 'R' {
            if r1 == std::usize::MAX {
                r1 = i;
            } else {
                r2 = i;
            }
        } else if s[i] == 'K' {
            k = i;
        }
    }
    if r1 < k && k < r2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
