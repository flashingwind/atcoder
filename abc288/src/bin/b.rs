use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
        k: usize,
        names: [String;n],
    };
    let mut topk = names.iter().take(k).collect::<Vec<_>>();
    topk.sort_by(|a, b| {
        // println!("{} {}?:", a, b);
        let mut itr_b = b.chars();
        for sa in a.chars() {
            if let Some(sb) = itr_b.next() {
                if sa < sb {
                    // println!("{}<{}", sa, sb);
                    return Ordering::Less;
                } else if sa > sb {
                    // println!("{}>{}", sa, sb);
                    return Ordering::Greater;
                } else if sa == sb {
                    // println!("{}=={}", sa, sb);
                    continue;
                }
            }
        }
        if a.len() < b.len() {
            // println!("|{}|<|{}|", a, b);
            return Ordering::Less;
        } else if a.len() > b.len() {
            // println!("|{}|>|{}|", a, b);
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
    });
    for name in topk.iter() {
        println!("{}", name);
    }
}
