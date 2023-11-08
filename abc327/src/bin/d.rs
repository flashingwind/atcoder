use proconio::{input, marker::Usize1};
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let mut idx_a = BTreeMap::default();
    let mut idx_b = BTreeMap::default();
    for _i in 0..m {
        input! {
            a: Usize1,
            b: Usize1,
        };
        idx_a.entry(a).or_insert(vec![]).push(b);
        idx_b.entry(b).or_insert(vec![]).push(a);
    }
    //init
    let mut x = vec![None; n];
    let mut next_a: Vec<usize> = vec![];
    let mut next_b: Vec<usize> = vec![];
    {
        let (a, b) = idx_a.first_key_value().unwrap();
        x[*a] = Some(true);
        next_a.push(*a);
        for bi in b.iter() {
            x[*bi] = Some(false);
            next_b.push(*bi);
        }
    }
    //check
    while next_a.len() != 0 || next_b.len() != 0 {
        for a in next_a.iter() {
            let b = idx_a.entry(*a).or_insert(vec![]);
            if let Some(fa) = x[*a] {
                for bi in b.iter() {
                    if let Some(fb) = x[*bi] {
                        if fa == fb {
                            println!("No");
                            return;
                        }
                    } else {
                        x[*bi] = Some(!fa);
                        next_b.push(*bi);
                    }
                }
            }
        }
        next_a.clear();
        for b in next_b.clone().iter() {
            let a = idx_b.entry(*b).or_insert(vec![]);
            if let Some(fb) = x[*b] {
                for ai in a.iter() {
                    if let Some(fa) = x[*ai] {
                        if fa == fb {
                            println!("No");
                            return;
                        }
                    } else {
                        x[*ai] = Some(!fb);
                        next_b.push(*ai);
                    }
                }
            }
        }
        next_b.clear();
    }
    println!("Yes");
}
