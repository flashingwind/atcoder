use std::vec;

use proconio::input;

fn main() {
    input! {
        n:usize,
        mut m:usize,
    };

    let mut edges = vec![Vec::new(); n + 1];
    for _ in 0..m {
        input! {
            mut n1: usize,
            mut n2: usize,
        };
        if n1 != n2 {
            // println!("{n1},{n2}");
            edges[n2].push(n1);
            edges[n1].push(n2);
        } else {
            m -= 1;
        }
    }

    let mut term = Vec::new();
    for (i, e) in edges.iter().enumerate().skip(1) {
        if e.len() == 1 {
            term.push(i);
        }
        if e.len() == 0 || 2 < e.len() {
            println!("No");
            return;
        }
    }
    if term.iter().count() != 2 {
        println!("No");
        return;
    }

    // println!("{:?}", edges);
    let mut i = term[0];
    let mut is_checked = vec![false; n + 1];
    is_checked[0] = true;
    for _ in 0..n {
        is_checked[i] = true;
        let mut is_found = false;
        // print!("{i}->{:?}", edges[i]);
        for e in edges[i].iter() {
            // println!("…{e}…");
            if *e != i && !is_checked[*e] {
                i = *e;
                is_found = true;
                break;
            }
        }
        // println!("->{i}");
        if !is_found {
            if i == term[1] && !is_checked.contains(&false) {
                println!("Yes");
            } else {
                println!("No");
            }
            return;
        }
    }
}
