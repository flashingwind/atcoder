use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut town_r: Chars,
        code: Chars,
    };
    let town = town_r.iter().map(|v| v.to_ascii_uppercase()).collect_vec();
    // println!("{:?}/{:?}", town, code);
    if code == town {
        println!("Yes");
        return;
    }
    let mut i = 0;
    loop {
        if i == town.len() {
            // println!("Over0: i={i}");
            println!("No");
            return;
        } else if code[0] == town[i] {
            // println!("  S0: {}=={}", code[0], town[i]);
            i += 1;
            break;
        } else {
            // println!("  S0: {}=={}", code[0], town[i]);
            i += 1;
        }
    }
    // println!("2nd i={i}");
    loop {
        if i == town.len() {
            // println!("Over1: i={i}");
            println!("No");
            return;
        } else if code[1] == town[i] {
            // println!("  S1: {}=={}", code[1], town[i]);
            i += 1;
            break;
        } else {
            // println!("  S1: {}=={}", code[1], town[i]);
            i += 1;
        }
    }
    // println!("3rd i={i}");
    if code[2] == 'X' {
        if i < town.len() {
            println!("Yes");
            return;
        }
        // println!("X mode: i={i} len={}", town.len());
    }
    loop {
        if i == town.len() {
            // println!("Over2: i={i}");
            println!("No");
            return;
        } else if code[2] == town[i] {
            // println!("  S2: {}=={}", code[2], town[i]);
            // i += 1;
            println!("Yes");
            return;
        } else {
            // println!("  S2: {}=={}", code[2], town[i]);
            i += 1;
        }
    }
    // // println!("No");
}
