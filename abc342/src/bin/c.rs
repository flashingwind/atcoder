use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
        q: usize,
        mut tr: [(char,char);q],
    };
    let mut cs: Vec<char> = vec![];
    let mut ls: Vec<usize> = vec![];
    for &c in s.iter() {
        if *cs.last().unwrap_or(&'X') == c {
            *ls.last_mut().unwrap() += 1;
        } else {
            cs.push(c);
            ls.push(1);
        }
    }
    let mut tr2: Vec<(char, char)> = vec![];
    let mut tr_index = vec![];
    for (i, &pair) in tr.iter().enumerate() {
        for &pair2 in tr2.iter() {
            if pair.0 == pair2.0 {
                tr_index.push(i);
            }
        }
    }
    for (i, &pair) in tr.iter().enumerate() {
        tr2.push(tr[i]);
    }
    // println!("cs: {:?}", cs);
    // println!("tr2: {:?}", tr2);
    for (c1, c2) in tr2.iter() {
        for c in cs.iter_mut() {
            if *c == *c1 {
                *c = *c2;
                // print!("{}->{}", c1,c);
            }
        }
    }
    let mut out = "".to_string();
    for (i, &c) in cs.iter().enumerate() {
        let len = ls[i];
        out += String::from(c).repeat(len).as_str();
        // println!("{}:{}", c, len);
    }
    println!("{}", out);
}
