use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        _: usize,
    };
    let mut prods = vec![Vec::new(); n];
    for i in 0..n {
        input! {
            p: u32,
            c: usize,
            f: [u32;c]
        };
        prods[i].push(p);
        prods[i].extend(f.iter().sorted().unique());
    }
    prods.sort();
    // println!("n={n}");
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let mut is_ok = true;
            // println!("P{i}={:?}", prods[i]);
            // println!("P{j}={:?}", prods[j]);
            // Pi=>Pj
            if !(prods[j][0] <= prods[i][0]) {
                is_ok = false;
                // println!(
                //     "{i} {j}: 1: prods[i][0] < prods[j][0]:{} {}",
                //     prods[i][0], prods[j][0]
                // );
            }
            // prod j doesnt have all funcs in prod i
            for k in 1..prods[i].len() {
                if !(prods[j][1..].contains(&prods[i][k])) {
                    is_ok = false;
                    // println!("{i} {j}: 2: prod j doesnt have all funcs in prod i");
                    break;
                }
            }
            // Pi <= Pj
            if prods[i][0] <= prods[j][0] {
                // println!("{i} {j}: 3: prods[i][0] <= prods[j][0]");
                // and prod i has all funcs which prod j has
                if prods[i].len() <= prods[j].len() {
                    is_ok = false;
                    // println!(
                    //     "{i} {j}: 3: prods[i].len():{} <= prods[j].len():{}",
                    //     prods[i].len(),
                    //     prods[j].len()
                    // );
                }
                for k in 1..prods[j].len() {
                    // println!(
                    //     "{i} {j}: 3: prods[i][1..]:{:?} contains? {}",
                    //     prods[i], prods[j][k]
                    // );
                    if !prods[i][1..].contains(&prods[j][k]) {
                        // println!(
                        //     "{i} {j}: 3: prods[i][1..]:{:?} not contains {}",
                        //     prods[i], prods[j][k]
                        // );
                        is_ok = false;
                        break;
                    }
                }
            }
            if is_ok {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
