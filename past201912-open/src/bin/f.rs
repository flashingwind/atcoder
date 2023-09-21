use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut w = Vec::new();
    let mut is_inside_word = false;
    for c in s.iter() {
        if !is_inside_word && c.is_uppercase() {
            w.push(vec![String::from(c.to_lowercase().to_string()); 1]);
            is_inside_word = true;
            // print!("{}", c);
        } else if is_inside_word && c.is_uppercase() {
            w.last_mut().unwrap().push(c.to_lowercase().to_string());
            is_inside_word = false;
            // println!("{}", c);
        } else {
            w.last_mut().unwrap().push(c.to_string());
            // print!("{}", c);
        }
    }
    w.sort();
    for line in w.iter() {
        print!("{}", line[0].to_uppercase());
        for c in line[1..line.len() - 1].iter() {
            print!("{}", c);
        }
        print!("{}", line.last().unwrap().to_uppercase());
    }
    println!();
}
