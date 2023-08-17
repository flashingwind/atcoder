use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: [Chars;10],
    };
    let mut startl = 0usize;

    for i in 0..10 {
        let mut is_all_dot = true;
        for c in s[i].iter() {
            if *c != '.' {
                is_all_dot = false;
            }
        }
        if is_all_dot {
        } else {
            startl = i + 1;
            break;
        }
    }
    let mut endl = startl;
    for i in (startl - 1..10).rev() {
        let mut is_all_dot = true;
        for c in s[i].iter() {
            if *c != '.' {
                is_all_dot = false;
            }
        }
        if is_all_dot {
        } else {
            endl = i + 1;
            break;
        }
    }

    let mut left = 0;
    let mut right = 0;
    let mut is_inside = false;
    for (j, c) in s[startl - 1].iter().enumerate() {
        if !is_inside && *c == '#' {
            left = j + 1;
            right = j + 1;
            is_inside = true;
        } else if is_inside && *c == '#' {
            right = j + 1;
        } else if is_inside && *c == '.' || 9 == j {
            is_inside = false;
        }
    }
    println!("{} {}", startl, endl);
    println!("{} {}", left, right);
}
