use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
        s: Chars
    };
    let mut sect = vec![];
    for c in s.iter() {
        if sect.len() == 0 {
            sect.push(vec![]);
        } else if sect.last().unwrap()[0] != *c {
            sect.push(vec![]);
        }
        sect.last_mut().unwrap().push(*c);
    }
    let mut cost = vec![];
    let mut is_caps = false;
    for (_i, ss) in sect.iter().enumerate() {
        let mut cst_caps = 0;
        let mut cst_shift = 0;
        if ss[0].is_lowercase() && !is_caps || ss[0].is_uppercase() && is_caps {
            cst_caps = x * ss.len();
            cst_shift = x * ss.len();
        } else if ss[0].is_lowercase() && is_caps || ss[0].is_uppercase() && !is_caps {
            cst_caps = z + x * ss.len(); //caps+a
            cst_shift = y * ss.len(); //(shift+a)
        }
        if cst_caps < cst_shift {
            is_caps = !is_caps;
        }
        cost.push((cst_caps, cst_shift));
    }
    let mut sum_min = std::usize::MAX;
    for pat in (0usize..sect.len()).permutations(2) {
        let mut sum = 0;
        for i in pat {
            for (c1, c2) in cost.iter() {
                if i == 0 {
                    sum += *c1;
                } else {
                    sum += *c2;
                }
                // println!("c1={c1} c2={c2}");
            }
        }
        if sum < sum_min {
            sum_min = sum;
        }
    }
    println!("{}", sum_min);
}
