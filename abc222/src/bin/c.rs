use std::cmp::Ordering;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        pred: [Chars;2*n],
    }
    // println!("{:?}", pred);
    let mut ps = vec![(0usize, 0u32); 2 * n];
    for (i, p) in ps.iter_mut().enumerate() {
        p.0 = i;
    }
    for t in 0..m {
        for k in 0..=n - 1 {
            // print!(
            //     "{}-{}: {}:{}-{}:{}",
            //     2 * k + 1,
            //     2 * k + 2,
            //     ps[2 * k].0 + 1,
            //     pred[ps[2 * k].0][t],
            //     ps[2 * k + 1].0 + 1,
            //     pred[ps[2 * k + 1].0][t]
            // );
            if let Some(a_win) = a_is_winner(pred[ps[2 * k].0][t], pred[ps[2 * k + 1].0][t]) {
                // println!(": {}", a_win);
                if a_win {
                    ps[2 * k].1 += 1;
                } else {
                    ps[2 * k + 1].1 += 1;
                }
            }
        }
        ps.sort_by(|a, b| match b.1.cmp(&a.1) {
            Ordering::Equal => a.0.cmp(&b.0),
            other => other,
        });
        for p in ps.iter_mut() {
            // println!("{}: {}", p.0 + 1, p.1);
        }
        // println!();
    }
    for p in ps.iter_mut() {
        println!("{}", p.0 + 1);
    }
}

fn a_is_winner(a: char, b: char) -> Option<bool> {
    if a == 'G' {
        if b == 'G' {
            return None;
        } else if b == 'C' {
            return Some(true);
        } else if b == 'P' {
            return Some(false);
        }
    } else if a == 'C' {
        if b == 'G' {
            return Some(false);
        } else if b == 'C' {
            return None;
        } else if b == 'P' {
            return Some(true);
        }
    } else if a == 'P' {
        if b == 'G' {
            return Some(true);
        } else if b == 'C' {
            return Some(false);
        } else if b == 'P' {
            return None;
        }
    }
    return None;
}
