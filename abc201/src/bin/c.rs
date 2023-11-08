use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let n = s.len();
    let mut must = vec![];
    let mut may = vec![];
    let mut must_not = vec![];
    for i in 0..n {
        if s[i] == 'o' {
            must.push(std::char::from_digit(i as u32, 10).unwrap());
        } else if s[i] == '?' {
            may.push(std::char::from_digit(i as u32, 10).unwrap());
        } else {
            must_not.push(std::char::from_digit(i as u32, 10).unwrap());
        }
    }
    if must.len() + may.len() == 0 || 4 < must.len() {
        println!("0");
        return;
    }
    let mut cnt = 0;
    for pn in 0..=9999 {
        let cs = format!("{:0>4}", pn).chars().collect_vec();
        // println!("{:?}", cs);
        let mut is_contains_must = vec![false; must.len()];
        let mut is_contains_must_not = false;
        for c in cs.iter() {
            if must_not.contains(c) {
                is_contains_must_not = true;
                break;
            }
            for i in 0..must.len() {
                if must[i] == *c {
                    is_contains_must[i] = true;
                }
            }
        }
        if is_contains_must.iter().all(|v| *v) && !is_contains_must_not {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
