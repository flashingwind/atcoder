use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut b: HashMap<char, bool> = HashMap::new();
    let mut res: bool = true;
    //let brkts_index: HashMap<usize, char> = HashMap::new();
    for (i, c) in s.iter().enumerate() {
        if *c == '(' {
        } else if *c == ')' {
            remove_from_box(&mut b, &s, i);
        } else {
            // alphabets
            if b.contains_key(c) {
                println!("b.contains_key({c})");
                res = false;
                break;
            } else {
                println!("b.insert({c}, true)");
                b.insert(*c, true);
            }
        }
    }
    if res {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn remove_from_box(b: &mut HashMap<char, bool>, s: &Vec<char>, i: usize) {
    let mut nested_lv = 0;
    for j in (0..=i).rev() {
        if s[j] == '(' {
            nested_lv -= 1;
        } else if s[j] == ')' {
            nested_lv += 1;
        } else {
            println!("    remove(s[j]={}): b={:?}", s[j], b);
            b.remove(&s[j]);
            println!("      removeed(s[j]={}): b={:?}", s[j], b);
        }
        if nested_lv == 0 {
            return;
        }
    }
}
